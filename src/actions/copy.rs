use anyhow::Result;
use log::info;
use serde::Deserialize;
use std::{fs, path::PathBuf};

use super::traits::Action;

#[derive(Deserialize, Clone)]
pub struct CopyActionArgs {
    pub src: PathBuf,
    pub dest: PathBuf,
}

pub struct CopyAction {
    args: CopyActionArgs,
}

impl CopyAction {
    pub fn new(args: CopyActionArgs) -> Self {
        Self { args }
    }
}

impl Action for CopyAction {
    fn condition(&self) -> bool {
        !self.args.dest.exists()
    }

    fn process(&self) -> Result<()> {
        fs::copy(&self.args.src, &self.args.dest)?;
        info!(
            "Copied file from {} to {}",
            self.args.src.display(),
            self.args.dest.display()
        );
        Ok(())
    }
}
