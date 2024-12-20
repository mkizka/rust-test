use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the tasks file
    #[arg(short, long)]
    pub file: String,

    /// Run in dry-run mode
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub dry_run: bool,
}

pub fn read_args() -> Args {
    Args::parse()
}
