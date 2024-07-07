use std::path::PathBuf;

use clap::Subcommand;

#[derive(Subcommand)]
#[command(arg_required_else_help = true)]
pub enum Command {
    /// Try to fix .fit file
    #[command(arg_required_else_help = true)]
    Fix {
        /// A .fit filename to fix
        #[arg(value_name = "FILE")]
        filename: PathBuf,
    },

    /// Display .fit file
    View {
        /// A .fit filename to display
        #[arg(value_name = "FILE")]
        filename: PathBuf,
    },
}
