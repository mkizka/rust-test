use anyhow::Result;
use serde::Deserialize;
use std::fs;

use crate::actions::copy::CopyActionArgs;
use crate::actions::mkdir::MkdirActionArgs;
use crate::actions::mv::MoveActionArgs;
use crate::actions::remove::RemoveActionArgs;
use crate::actions::shell::ShellActionArgs;

#[derive(Deserialize)]
#[serde(tag = "action", rename_all = "lowercase")]
pub enum ActionDefinition {
    Copy { args: CopyActionArgs },
    Remove { args: RemoveActionArgs },
    Mkdir { args: MkdirActionArgs },
    Move { args: MoveActionArgs },
    Shell { args: ShellActionArgs },
}

#[derive(Deserialize)]
pub struct TaskDefinition {
    pub name: String,
    #[serde(flatten)]
    pub action: ActionDefinition,
}

#[derive(Deserialize)]
pub struct Yaml {
    pub tasks: Vec<TaskDefinition>,
}

pub fn read_yaml(path: &str) -> Result<Yaml> {
    let content = fs::read_to_string(path)?;
    let yaml: Yaml = serde_yaml::from_str(&content)?;
    Ok(yaml)
}
