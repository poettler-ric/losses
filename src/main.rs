use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process;

/// Keeps and evaluates records of why chess games were lost
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Command to execute
    #[command(subcommand)]
    command: Commands,
}

/// CLI subcommands
#[derive(Debug, Subcommand)]
enum Commands {
    /// Adds a result to the record
    Add { cause: losses::Cause },
    /// Summarizes recorded results
    Summarize,
}

fn main() {
    let csv_file = match dirs::data_dir() {
        Some(data_dir) => data_dir.join("losses").join("games.csv"),
        None => PathBuf::from("games.csv"),
    };
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { cause } => {
            if let Err(e) = losses::add(cause, &csv_file) {
                eprintln!("Error while adding to {:?}: {}", csv_file, e);
                process::exit(1);
            }
        }
        Commands::Summarize => {
            if let Err(e) = losses::summarize(&csv_file) {
                eprintln!("Error while summarizing {:?}: {}", csv_file, e);
                process::exit(2);
            }
        }
    };
}
