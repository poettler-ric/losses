use chrono::{DateTime, Duration, Utc};
use clap::ValueEnum;
use csv::{ReaderBuilder, WriterBuilder};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::OpenOptions;
use std::path::Path;

/// Cause for the lost game
#[derive(Debug, Deserialize, Serialize, ValueEnum, Clone, Derivative, Eq)]
#[derivative(Hash, PartialEq)]
pub enum Cause {
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
pub struct Game {
    /// Date of the game
    date: DateTime<Utc>,
    /// Cause for the loss
    cause: Cause,
}

pub fn add(cause: Cause, filename: &Path) -> Result<(), Box<dyn Error>> {
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

pub fn summarize(filename: &Path) -> Result<(), Box<dyn Error>> {
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
