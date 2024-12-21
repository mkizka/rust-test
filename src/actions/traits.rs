use crate::arg::Args;
use anyhow::Result;
use log::{error, info};

pub trait Action {
    fn condition(&self) -> bool;
    fn process(&self) -> Result<()>;

    fn run(&self, task: &crate::yaml::TaskDefinition, args: &Args) {
        if self.condition() {
            if args.dry_run {
                info!(
                    "[DRY-RUN] Condition met for task '{}', action would be executed.",
                    task.name
                );
            } else {
                info!("Executing task '{}'", task.name);
                if let Err(e) = self.process() {
                    error!("Task '{}' failed: {}", task.name, e);
                }
            }
        } else {
            info!("Condition not met for task '{}', skipping.", task.name);
        }
    }
}
