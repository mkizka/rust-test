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
    fn condition(&self) -> bool;
    fn action(&self);
    fn execute_task(&self, dry_run_mode: bool, task: &Task) {
        if self.condition() {
            if dry_run_mode {
                println!(
                    "[DRY-RUN] Condition met for task '{}', action would be executed.",
                    task.name
                );
            } else {
                self.action();
            }
        } else {
            println!(
                "Condition not met for task '{}', skipping action.",
                task.name
            );
        }
    }
}

struct FileTask {
    path: String,
}

impl FileTask {
    fn from_yaml(args: &serde_yaml::Value) -> Self {
        FileTask {
            path: args["path"]
                .as_str()
                .expect("Missing 'path' argument")
                .to_string(),
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
    fn from_yaml(args: &serde_yaml::Value) -> Self {
        CopyTask {
            src: args["src"]
                .as_str()
                .expect("Missing 'src' argument")
                .to_string(),
            dest: args["dest"]
                .as_str()
                .expect("Invalid 'dest' argument")
                .to_string(),
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
            "file" => Box::new(FileTask::from_yaml(&task.args)),
            "copy" => Box::new(CopyTask::from_yaml(&task.args)),
            _ => {
                println!("Unknown action: {}", task.action);
                return;
            }
        };

        strategy.execute_task(dry_run_mode, task);
    });
}
