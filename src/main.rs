mod arg;
mod strategy;
mod yaml;

use arg::read_args;
use std::process;
use strategy::create_strategy;
use yaml::read_yaml;

fn main() {
    let args = read_args();

    let yaml = read_yaml(&args.file).unwrap_or_else(|e| {
        eprintln!("Failed to read workflow file: {}", e);
        process::exit(1);
    });

    yaml.tasks.iter().for_each(|task| {
        create_strategy(task).run(task, &args);
    });
}
