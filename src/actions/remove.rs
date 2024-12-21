use anyhow::Result;
use log::info;
use serde::Deserialize;
use std::{fs, path::PathBuf};

use super::traits::Action;

#[derive(Deserialize, Clone)]
pub struct RemoveActionArgs {
    pub path: PathBuf,
}

pub struct RemoveAction {
    args: RemoveActionArgs,
}

impl RemoveAction {
    pub fn new(args: RemoveActionArgs) -> Self {
        Self { args }
    }
}

impl Action for RemoveAction {
    fn condition(&self) -> bool {
        self.args.path.exists()
    }

    fn process(&self) -> Result<()> {
        fs::remove_file(&self.args.path)?;
        info!("Removed file: {}", self.args.path.display());
        Ok(())
    }
}
