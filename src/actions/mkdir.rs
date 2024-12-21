use anyhow::Result;
use log::info;
use serde::Deserialize;
use std::{fs, path::PathBuf};

use super::traits::Action;

#[derive(Deserialize, Clone)]
pub struct MkdirActionArgs {
    pub path: PathBuf,
}

pub struct MkdirAction {
    args: MkdirActionArgs,
}

impl MkdirAction {
    pub fn new(args: MkdirActionArgs) -> Self {
        Self { args }
    }
}

impl Action for MkdirAction {
    fn condition(&self) -> bool {
        !self.args.path.exists()
    }

    fn process(&self) -> Result<()> {
        fs::create_dir_all(&self.args.path)?;
        info!("Created directory: {}", self.args.path.display());
        Ok(())
    }
}
