use chrono::{DateTime, Duration, Utc};
use clap::{Parser, Subcommand, ValueEnum};
use csv::{ReaderBuilder, WriterBuilder};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::process;

/// Cause for the lost game
#[derive(Debug, Deserialize, Serialize, ValueEnum, Clone, Derivative, Eq)]
#[derivative(Hash, PartialEq)]
enum Cause {
    /// Game was lost in the opening
    Opening,
    /// Game was lost in the middlegame
    Middlegame,
    /// Game was lost in the endgame
    Endgame,
    /// Game was lost on time
    Time,
    /// Game was gradually lost due to missing strategy
    Strategy,
}

/// Record of lost game
#[derive(Debug, Deserialize, Serialize)]
struct Game {
    /// Date of the game
    date: DateTime<Utc>,
    /// Cause for the loss
    cause: Cause,
}

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
    Add { cause: Cause },
    /// Summarizes recorded results
    Summarize,
}

fn add(cause: Cause, filename: &Path) -> Result<(), Box<dyn Error>> {
    let game = Game {
        date: Utc::now(),
        cause,
    };
    if let Some(parent) = filename.parent() {
        if !parent.is_dir() {
            std::fs::create_dir_all(parent)?;
        }
    }
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(filename)?;
    let mut writer = WriterBuilder::new().has_headers(false).from_writer(file);
    writer.serialize(game)?;
    writer.flush()?;
    Ok(())
}

fn summarize(filename: &Path) -> Result<(), Box<dyn Error>> {
    let before_thirty_days = Utc::now() - Duration::days(30);
    let mut summary: HashMap<Cause, u32> = HashMap::new();
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_path(filename)?;
    for record in reader.deserialize() {
        let game: Game = record?;
        if game.date > before_thirty_days {
            summary
                .entry(game.cause)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }
    let mut sorted: Vec<(&Cause, &u32)> = summary.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    for (cause, count) in sorted {
        println!("{:?}: {}", cause, count);
    }
    Ok(())
}

fn main() {
    let csv_file = match dirs::data_dir() {
        Some(data_dir) => data_dir.join("losses").join("games.csv"),
        None => PathBuf::from("games.csv"),
    };
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { cause } => {
            if let Err(e) = add(cause, &csv_file) {
                eprintln!("Error while adding to {:?}: {}", csv_file, e);
                process::exit(1);
            }
        }
        Commands::Summarize => {
            if let Err(e) = summarize(&csv_file) {
                eprintln!("Error while summarizing {:?}: {}", csv_file, e);
                process::exit(2);
            }
        }
    };
}
