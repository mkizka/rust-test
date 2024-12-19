use clap::{Arg, ArgAction, Command};
use serde::Deserialize;
use std::{fs, ops::Not, path::Path};

#[derive(Deserialize)]
struct Task {
    name: String,
    action: String,
    args: serde_yaml::Value,
}

#[derive(Deserialize)]
struct Playbook {
    tasks: Vec<Task>,
}

trait TaskStrategy {
    fn condition(&self, args: &serde_yaml::Value) -> bool;
    fn action(&self, args: &serde_yaml::Value);
}

struct FileTask;
impl TaskStrategy for FileTask {
    fn condition(&self, args: &serde_yaml::Value) -> bool {
        Path::new(args["path"].as_str().expect("Missing 'path' argument"))
            .exists()
            .not()
    }

    fn action(&self, args: &serde_yaml::Value) {
        let path = args["path"].as_str().expect("Invalid 'path' argument");
        fs::create_dir_all(path).expect("Failed to create directory");
        println!("Created directory: {}", path);
    }
}

struct CopyTask;
impl TaskStrategy for CopyTask {
    fn condition(&self, args: &serde_yaml::Value) -> bool {
        !Path::new(args["dest"].as_str().expect("Missing 'dest' argument")).exists()
    }

    fn action(&self, args: &serde_yaml::Value) {
        let src = args["src"].as_str().expect("Missing 'src' argument");
        let dest = args["dest"].as_str().expect("Invalid 'dest' argument");
        fs::copy(src, dest).expect("Failed to copy file");
        println!("Copied file from {} to {}", src, dest);
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
            "file" => Box::new(FileTask),
            "copy" => Box::new(CopyTask),
            _ => {
                println!("Unknown action: {}", task.action);
                return;
            }
        };

        execute_task(&*strategy, &task.args, dry_run_mode, task);
    });
}

fn execute_task(
    strategy: &dyn TaskStrategy,
    args: &serde_yaml::Value,
    dry_run_mode: bool,
    task: &Task,
) {
    if strategy.condition(args) {
        if dry_run_mode {
            println!("[DRY-RUN] Condition met for task '{}', action would be executed with arguments: {:?}", task.name, task.args);
        } else {
            strategy.action(args);
        }
    } else {
        println!(
            "Condition not met for task '{}', skipping action.",
            task.name
        );
    }
}
