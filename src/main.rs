use std::ffi::OsString;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, error::ErrorKind};
use modm::init_modm;

mod utils;
use utils::authenticate_github;

// A git like cli tool for machine learning version control 
#[derive(Debug, Parser)] // requires `derive` feature
#[clap(name = "modm", version = "0.1.0", author = "Maksym Vovk")]
#[clap(about = "A git like cli tool for machine learning version control ", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Clones repos
    #[clap(arg_required_else_help = true)]
    Clone {
        /// The remote to clone
        remote: String,
    },

    // Initializes the modm repository in the .modm folder
    Init,
    // Checks if the user is logged in and if not, prompts the user to login
    Auth,
}

fn main() -> Result <(), ErrorKind> {
    let args = Cli::parse();

    match args.command {
        Commands::Clone { remote } => {
            println!("Cloning {}", remote);
        }
        Commands::Init => {
           if let Err(e) = init_modm() {
               println!("Error: {}", e);
           }
        }
        Commands::Auth => {
            authenticate_github();
        }
    }

    Ok(())
}