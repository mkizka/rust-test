mod actions;
mod arg;
mod yaml;

use actions::create_action;
use anyhow::Result;
use arg::read_args;
use log::info;
use std::env;
use yaml::read_yaml;

fn main() -> Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let args = read_args();
    let yaml = read_yaml(&args.file)?;

    info!("Starting task execution");
    for task in &yaml.tasks {
        create_action(&task.action).run(task, &args);
    }

    Ok(())
}
