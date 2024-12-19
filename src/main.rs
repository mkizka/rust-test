use clap::Parser;
use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Deserialize)]
struct Task {
    name: String,
    action: String,
    args: TaskArgs,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum TaskArgs {
    File { path: String },
    Copy { src: String, dest: String },
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
    fn new(path: String) -> Self {
        FileTask { path }
    }
}

impl TaskStrategy for FileTask {
    fn condition(&self) -> bool {
        !Path::new(&self.path).exists()
    }

    fn action(&self) {
        if let Err(e) = fs::create_dir_all(&self.path) {
            eprintln!("Failed to create directory '{}': {}", self.path, e);
        } else {
            println!("Created directory: {}", self.path);
        }
    }
}

struct CopyTask {
    src: String,
    dest: String,
}

impl CopyTask {
    fn new(src: String, dest: String) -> Self {
        CopyTask { src, dest }
    }
}

impl TaskStrategy for CopyTask {
    fn condition(&self) -> bool {
        !Path::new(&self.dest).exists()
    }

    fn action(&self) {
        if let Err(e) = fs::copy(&self.src, &self.dest) {
            eprintln!(
                "Failed to copy file from '{}' to '{}': {}",
                self.src, self.dest, e
            );
        } else {
            println!("Copied file from {} to {}", self.src, self.dest);
        }
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

    let file = match fs::read_to_string(&args.file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Unable to read the playbook file '{}': {}", args.file, e);
            return;
        }
    };

    let workflow: Workflow = match serde_yaml::from_str(&file) {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("Invalid YAML format in file '{}': {}", args.file, e);
            return;
        }
    };

    workflow.tasks.iter().for_each(|task| {
        if args.verbose {
            println!("Executing task: {}", task.name);
        }

        let strategy: Box<dyn TaskStrategy> = match &task.args {
            TaskArgs::File { path } => Box::new(FileTask::new(path.clone())),
            TaskArgs::Copy { src, dest } => Box::new(CopyTask::new(src.clone(), dest.clone())),
        };

        strategy.execute_task(args.dry_run, task);
    });
}
