use clap::{Args, Parser, Subcommand};

/// Basalt is a set of utilities for knowledge management in the terminal.
#[derive(Parser, Debug)]
#[command(version)]
pub struct App {
    /// The command you want to run.
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
}
