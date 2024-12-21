pub mod copy;
pub mod mkdir;
pub mod mv;
pub mod remove;
pub mod shell;
mod traits;

pub use copy::CopyAction;
pub use mkdir::MkdirAction;
pub use mv::MoveAction;
pub use remove::RemoveAction;
pub use shell::ShellAction;
pub use traits::Action;

use crate::yaml::ActionDefinition;

pub fn create_action(action: &ActionDefinition) -> Box<dyn Action> {
    match &action {
        ActionDefinition::Copy { args } => Box::new(CopyAction::new(args.clone())),
        ActionDefinition::Remove { args } => Box::new(RemoveAction::new(args.clone())),
        ActionDefinition::Mkdir { args } => Box::new(MkdirAction::new(args.clone())),
        ActionDefinition::Move { args } => Box::new(MoveAction::new(args.clone())),
        ActionDefinition::Shell { args } => Box::new(ShellAction::new(args.clone())),
    }
}
