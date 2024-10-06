use crate::magic::DEFAULT_OUTPUT_PATH;
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
    /// Initialize a vault in the current directory.
    ///
    /// Sets up the necessary structure to track
    /// config, build documents, store state, etc.
    Init,

    /// Compile a vault.
    Compile(CompileArgs),

    /// Watch a vault.
    Watch,

    /// Perform configuration related actions.
    #[clap(subcommand)]
    Config(ConfigCommands),
}

#[derive(Args, Debug)]
pub struct CompileArgs {
    // /// Path of vault to compile.
    // #[clap(long, default_value = ".")]
    // pub path: String,
    /// Path to write result.
    #[clap(long, default_value = DEFAULT_OUTPUT_PATH)]
    pub output: String,
    /// Program to open result with.
    #[clap(long)]
    pub open: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Set config values.
    ///
    /// Overwrites existing values, if present.
    Set(ConfigSetArgs),

    /// Get config values.
    Get(ConfigGetArgs),

    /// Print entire config.
    Print(ConfigPrintArgs),
}

#[derive(Args, Debug)]
pub struct ConfigSetArgs {
    /// Which option to set.
    pub option: String,

    /// Value to set option to.
    pub value: String,
}

#[derive(Args, Debug)]
pub struct ConfigGetArgs {
    /// Which option to get.
    pub option: String,
}

#[derive(Args, Debug)]
pub struct ConfigPrintArgs {
    /// Prints default config.
    #[clap(long, short, action, conflicts_with = "full")]
    pub default: bool,

    /// Prints full config, including default values.
    #[clap(long, short, action, conflicts_with = "default")]
    pub full: bool,
}
