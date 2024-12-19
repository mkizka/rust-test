use clap::{Arg, ArgAction, Command};
use serde::Deserialize;
use std::any::Any;
use std::{fs, path::Path};

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

#[derive(Debug)]
struct FileTaskContext {
    path: String,
}

impl FileTaskContext {
    fn from_yaml(value: &serde_yaml::Value) -> Result<Self, String> {
        let path = value["path"]
            .as_str()
            .ok_or_else(|| "Missing or invalid 'path' argument".to_string())?
            .to_string();
        Ok(Self { path })
    }
}

#[derive(Debug)]
struct CopyTaskContext {
    src: String,
    dest: String,
}

impl CopyTaskContext {
    fn from_yaml(value: &serde_yaml::Value) -> Result<Self, String> {
        let src = value["src"]
            .as_str()
            .ok_or_else(|| "Missing or invalid 'src' argument".to_string())?
            .to_string();
        let dest = value["dest"]
            .as_str()
            .ok_or_else(|| "Missing or invalid 'dest' argument".to_string())?
            .to_string();
        Ok(Self { src, dest })
    }
}

trait TaskStrategy {
    fn validate_args(&self, args: &serde_yaml::Value) -> Result<(), String>;
    fn condition(&self, context: &dyn TaskContext) -> bool;
    fn action(&self, context: &dyn TaskContext);
}

trait TaskContext: Any + std::fmt::Debug {}

impl TaskContext for FileTaskContext {}
impl TaskContext for CopyTaskContext {}

struct FileTask;
impl TaskStrategy for FileTask {
    fn validate_args(&self, args: &serde_yaml::Value) -> Result<(), String> {
        FileTaskContext::from_yaml(args)?;
        Ok(())
    }

    fn condition(&self, context: &dyn TaskContext) -> bool {
        // downcast_refを使って具体的な型にキャストする
        let context = context
            .downcast_ref::<FileTaskContext>()
            .expect("Invalid context for FileTask");
        !Path::new(&context.path).exists()
    }

    fn action(&self, context: &dyn TaskContext) {
        let context = context
            .downcast_ref::<FileTaskContext>()
            .expect("Invalid context for FileTask");
        fs::create_dir_all(&context.path).expect("Failed to create directory");
        println!("Created directory: {}", context.path);
    }
}

struct CopyTask;
impl TaskStrategy for CopyTask {
    fn validate_args(&self, args: &serde_yaml::Value) -> Result<(), String> {
        CopyTaskContext::from_yaml(args)?;
        Ok(())
    }

    fn condition(&self, context: &dyn TaskContext) -> bool {
        let context = context
            .downcast_ref::<CopyTaskContext>()
            .expect("Invalid context for CopyTask");
        !Path::new(&context.dest).exists()
    }

    fn action(&self, context: &dyn TaskContext) {
        let context = context
            .downcast_ref::<CopyTaskContext>()
            .expect("Invalid context for CopyTask");
        fs::copy(&context.src, &context.dest).expect("Failed to copy file");
        println!("Copied file from {} to {}", context.src, context.dest);
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

        // 引数の検証を行う
        match strategy.validate_args(&task.args) {
            Ok(_) => {
                // Contextを作成
                let context: Box<dyn TaskContext> = match task.action.as_str() {
                    "file" => Box::new(
                        FileTaskContext::from_yaml(&task.args)
                            .expect("Invalid arguments for FileTask"),
                    ),
                    "copy" => Box::new(
                        CopyTaskContext::from_yaml(&task.args)
                            .expect("Invalid arguments for CopyTask"),
                    ),
                    _ => {
                        println!("Unknown action: {}", task.action);
                        return;
                    }
                };

                execute_task(&*strategy, &*context, dry_run_mode, task);
            }
            Err(e) => println!("Task '{}' failed argument validation: {}", task.name, e),
        }
    });
}

fn execute_task(
    strategy: &dyn TaskStrategy,
    context: &dyn TaskContext,
    dry_run_mode: bool,
    task: &Task,
) {
    if strategy.condition(context) {
        if dry_run_mode {
            println!("[DRY-RUN] Condition met for task '{}', action would be executed with context: {:?}", task.name, context);
        } else {
            strategy.action(context);
        }
    } else {
        println!(
            "Condition not met for task '{}', skipping action.",
            task.name
        );
    }
}
