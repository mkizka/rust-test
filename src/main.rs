use clap::Parser;
use serde::Deserialize;
use std::{fs, ops::Not, path::Path};

#[derive(Deserialize)]
struct Task {
    name: String,
    action: String,
    args: serde_yaml::Value,
}

#[derive(Deserialize)]
struct Workflow {
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

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the tasks file
    #[arg(short, long)]
    file: String,

    /// Run in dry-run mode
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    dry_run: bool,

    /// Show verbose output
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    let file = fs::read_to_string(&args.file).expect("Unable to read the playbook file");
    let workflow: Workflow = serde_yaml::from_str(&file).expect("Invalid YAML format");

    workflow.tasks.iter().for_each(|task| {
        if args.verbose {
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

        strategy.execute_task(args.dry_run, task);
    });
}
