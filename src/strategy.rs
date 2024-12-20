use std::{fs, path::Path};

use crate::{
    arg::Args,
    yaml::{Action, Task},
};

pub trait Strategy {
    fn condition(&self) -> bool;
    fn action(&self);
    fn run(&self, task: &Task, args: &Args) {
        if self.condition() {
            if args.dry_run {
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

pub struct FileStrategy {
    pub path: String,
}

impl Strategy for FileStrategy {
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

pub struct CopyStrategy {
    pub src: String,
    pub dest: String,
}

impl Strategy for CopyStrategy {
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

pub fn create_strategy(task: &Task) -> Box<dyn Strategy> {
    match &task.action {
        Action::File { args } => Box::new(FileStrategy {
            path: args.path.clone(),
        }),
        Action::Copy { args } => Box::new(CopyStrategy {
            src: args.src.clone(),
            dest: args.dest.clone(),
        }),
    }
}
