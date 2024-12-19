use clap::{Arg, ArgAction, Command};
use serde::Deserialize;
use std::{fs, ops::Not, path::Path};

#[derive(Deserialize, Clone)] // Clone traitを追加
struct Task {
    name: String,
    action: String,
    args: TaskArgs,
}

#[derive(Deserialize, Clone)] // Clone traitを追加
struct TaskArgs {
    path: Option<String>,
    src: Option<String>,
    dest: Option<String>,
}

#[derive(Deserialize)]
struct Playbook {
    tasks: Vec<Task>,
}

trait TaskStrategy {
    fn condition(&self) -> bool;
    fn action(&self);
}

struct FileTask {
    path: String,
}

impl FileTask {
    fn from_yaml(args: TaskArgs) -> Self {
        FileTask {
            path: args.path.expect("Missing 'path' argument"),
        }
    }
}

impl TaskStrategy for FileTask {
    fn condition(&self) -> bool {
        Path::new(&self.path).exists().not()
    }

    fn action(&self) {
        fs::create_dir_all(&self.path).expect("Failed to create directory");
        println!("Created directory: {}", self.path);
    }
}

struct CopyTask {
    src: String,
    dest: String,
}

impl CopyTask {
    fn from_yaml(args: TaskArgs) -> Self {
        CopyTask {
            src: args.src.expect("Missing 'src' argument"),
            dest: args.dest.expect("Missing 'dest' argument"),
        }
    }
}

impl TaskStrategy for CopyTask {
    fn condition(&self) -> bool {
        !Path::new(&self.dest).exists()
    }

    fn action(&self) {
        fs::copy(&self.src, &self.dest).expect("Failed to copy file");
        println!("Copied file from {} to {}", self.src, self.dest);
    }
}

fn main() {
    let matches = Command::new("idempot-cli")
        .version("1.0")
        .about("A lightweight CLI for idempotent operations")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Specifies the playbook file")
                .required(true),
        )
        .arg(
            Arg::new("dry-run")
                .short('d')
                .long("dry-run")
                .help("Runs in dry-run mode without making changes")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enables verbose output")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let file = matches
        .get_one::<String>("file")
        .expect("File argument is required");
    let dry_run_mode = matches.get_flag("dry-run");
    let verbose = matches.get_flag("verbose");

    let playbook_content = fs::read_to_string(file).expect("Unable to read the playbook file");
    let playbook: Playbook = serde_yaml::from_str(&playbook_content).expect("Invalid YAML format");

    playbook.tasks.iter().for_each(|task| {
        if verbose {
            println!("Executing task: {}", task.name);
        }

        let strategy: Box<dyn TaskStrategy> = match task.action.as_str() {
            "file" => Box::new(FileTask::from_yaml(task.args.clone())),
            "copy" => Box::new(CopyTask::from_yaml(task.args.clone())),
            _ => {
                println!("Unknown action: {}", task.action);
                return;
            }
        };

        execute_task(&*strategy, dry_run_mode, task);
    });
}

fn execute_task(strategy: &dyn TaskStrategy, dry_run_mode: bool, task: &Task) {
    if strategy.condition() {
        if dry_run_mode {
            println!(
                "[DRY-RUN] Condition met for task '{}', action would be executed.",
                task.name
            );
        } else {
            strategy.action();
        }
    } else {
        println!(
            "Condition not met for task '{}', skipping action.",
            task.name
        );
    }
}
