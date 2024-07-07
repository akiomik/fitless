use anyhow::Result;
use clap::Parser;

use fitless::{
    cli::{Cli, Command},
    service::{Fixer, Viewer},
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::Fix { filename }) => {
            let fixer = Fixer::new();
            fixer.fix(filename.as_path())
        }
        Some(Command::View { filename }) => {
            let viewer = Viewer::new();
            viewer.display(filename.as_path())
        }
        None => Ok(()),
    }
}
