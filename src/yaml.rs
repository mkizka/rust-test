use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Deserialize)]
pub struct FileActionArgs {
    pub path: String,
}

#[derive(Deserialize)]
pub struct CopyActionArgs {
    pub src: String,
    pub dest: String,
}

#[derive(Deserialize)]
pub struct Task {
    pub name: String,
    #[serde(flatten)]
    pub action: Action,
}

#[derive(Deserialize)]
#[serde(tag = "action", rename_all = "lowercase")]
pub enum Action {
    File { args: FileActionArgs },
    Copy { args: CopyActionArgs },
}

#[derive(Deserialize)]
pub struct Yaml {
    pub tasks: Vec<Task>,
}

pub fn read_yaml(path: &str) -> Result<Yaml, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let yaml: Yaml = serde_yaml::from_str(&content)?;
    Ok(yaml)
}
