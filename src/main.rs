use clap::{Arg, ArgAction, Command};
use serde::Deserialize;
use std::fs;
use std::process::Command as ProcessCommand;

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
        !std::path::Path::new(args["path"].as_str().expect("Missing 'path' argument")).exists()
    }

    fn action(&self, args: &serde_yaml::Value) {
        let path = args["path"].as_str().unwrap();
        fs::create_dir_all(path).expect("Failed to create directory");
        println!("Created directory: {}", path);
    }
}

struct CopyTask;
impl TaskStrategy for CopyTask {
    fn condition(&self, args: &serde_yaml::Value) -> bool {
        let dest = args["dest"].as_str().expect("Missing 'dest' argument");
        !std::path::Path::new(dest).exists()
    }

    fn action(&self, args: &serde_yaml::Value) {
        let src = args["src"].as_str().expect("Missing 'src' argument");
        let dest = args["dest"].as_str().unwrap();
        fs::copy(src, dest).expect("Failed to copy file");
        println!("Copied file from {} to {}", src, dest);
    }
}

struct PackageTask;
impl TaskStrategy for PackageTask {
    fn condition(&self, args: &serde_yaml::Value) -> bool {
        let name = args["name"].as_str().expect("Missing 'name' argument");
        !is_package_installed(name)
    }

    fn action(&self, args: &serde_yaml::Value) {
        let name = args["name"].as_str().unwrap();
        ProcessCommand::new("apt-get")
            .args(["install", "-y", name])
            .status()
            .expect("Failed to install package");
        println!("Installed package: {}", name);
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
            Arg::new("check")
                .short('c')
                .long("check")
                .help("Runs in check mode without making changes")
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

    let file = matches.get_one::<String>("file").unwrap();
    let check_mode = matches.get_flag("check");
    let verbose = matches.get_flag("verbose");

    let playbook_content = fs::read_to_string(file).expect("Unable to read the playbook file");
    let playbook: Playbook = serde_yaml::from_str(&playbook_content).expect("Invalid YAML format");

    for task in playbook.tasks {
        if verbose {
            println!("Executing task: {}", task.name);
        }

        let strategy: Box<dyn TaskStrategy> = match task.action.as_str() {
            "file" => Box::new(FileTask),
            "copy" => Box::new(CopyTask),
            "package" => Box::new(PackageTask),
            _ => {
                println!("Unknown action: {}", task.action);
                continue;
            }
        };

        execute_task(strategy.as_ref(), &task.args, check_mode);
    }
}

fn execute_task(strategy: &dyn TaskStrategy, args: &serde_yaml::Value, check_mode: bool) {
    if strategy.condition(args) {
        if check_mode {
            println!("[CHECK] Condition met, action would be executed.");
        } else {
            strategy.action(args);
        }
    } else {
        println!("Condition not met, skipping action.");
    }
}

fn is_package_installed(name: &str) -> bool {
    let output = ProcessCommand::new("dpkg-query")
        .args(["-W", "--showformat='${Status}'", name])
        .output()
        .expect("Failed to query package status");

    String::from_utf8_lossy(&output.stdout).contains("install ok installed")
}
