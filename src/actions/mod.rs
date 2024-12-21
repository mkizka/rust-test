mod copy;
mod file;
mod traits;

pub use copy::CopyAction;
pub use file::FileAction;
pub use traits::Action;

use crate::yaml::ActionDefinition;

pub fn create_action(action: &ActionDefinition) -> Box<dyn Action> {
    match &action {
        ActionDefinition::File { args } => Box::new(FileAction::new(args.clone())),
        ActionDefinition::Copy { args } => Box::new(CopyAction::new(args.clone())),
    }
}
