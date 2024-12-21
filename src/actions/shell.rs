use anyhow::Result;
use log::info;
use serde::Deserialize;
use std::process::Command;

use super::traits::Action;

#[derive(Deserialize, Clone)]
pub struct ShellActionArgs {
    pub command: String,
}

pub struct ShellAction {
    args: ShellActionArgs,
}

impl ShellAction {
    pub fn new(args: ShellActionArgs) -> Self {
        Self { args }
    }
}

impl Action for ShellAction {
    fn condition(&self) -> bool {
        true
    }

    fn process(&self) -> Result<()> {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", &self.args.command])
                .output()?
        } else {
            Command::new("sh")
                .args(["-c", &self.args.command])
                .output()?
        };

        info!("Executed command: {}", self.args.command);

        if !output.stdout.is_empty() {
            info!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        }
        if !output.stderr.is_empty() {
            info!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        }

        Ok(())
    }
}
