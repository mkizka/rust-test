use anyhow::Result;
use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Deserialize, Clone)]
pub struct FileActionArgs {
    pub path: PathBuf,
}

#[derive(Deserialize, Clone)]
pub struct CopyActionArgs {
    pub src: PathBuf,
    pub dest: PathBuf,
}

#[derive(Deserialize)]
#[serde(tag = "action", rename_all = "lowercase")]
pub enum ActionDefinition {
    File { args: FileActionArgs },
    Copy { args: CopyActionArgs },
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
