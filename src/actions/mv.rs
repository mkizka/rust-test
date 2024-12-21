use anyhow::Result;
use log::info;
use serde::Deserialize;
use std::{fs, path::PathBuf};

use super::traits::Action;

#[derive(Deserialize, Clone)]
pub struct MoveActionArgs {
    pub src: PathBuf,
    pub dest: PathBuf,
}

pub struct MoveAction {
    args: MoveActionArgs,
}

impl MoveAction {
    pub fn new(args: MoveActionArgs) -> Self {
        Self { args }
    }
}

impl Action for MoveAction {
    fn condition(&self) -> bool {
        self.args.src.exists() && !self.args.dest.exists()
    }

    fn process(&self) -> Result<()> {
        fs::rename(&self.args.src, &self.args.dest)?;
        info!(
            "Moved file from {} to {}",
            self.args.src.display(),
            self.args.dest.display()
        );
        Ok(())
    }
}
