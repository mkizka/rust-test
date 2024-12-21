use anyhow::Result;
use log::info;
use std::fs;

use super::traits::Action;
use crate::yaml::FileActionArgs;

pub struct FileAction {
    args: FileActionArgs,
}

impl FileAction {
    pub fn new(args: FileActionArgs) -> Self {
        Self { args }
    }
}

impl Action for FileAction {
    fn condition(&self) -> bool {
        !self.args.path.exists()
    }

    fn process(&self) -> Result<()> {
        fs::create_dir_all(&self.args.path)?;
        info!("Created directory: {}", self.args.path.display());
        Ok(())
    }
}
