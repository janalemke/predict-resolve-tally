use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Record a prediction by answering some prompts
    Predict,
    /// Resolve any due predictions
    Resolve,
    /// Show your accuracy for various probability bins
    Tally,
    /// List all predictions
    Show,
    /// Calculate Brier Score (Calibration) of your predictions
    Score,
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}
