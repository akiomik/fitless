use std::error::Error;

use clap::Parser;

use fitless::{
    cli::{Cli, Command},
    service::fixer::Fixer,
};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::Fix { filename }) => {
            let fixer = Fixer::new();
            fixer.fix(filename)
        }
        None => Ok(()),
    }
}
