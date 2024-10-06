mod app;

use crate::app::{App, Command::*};
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = App::parse();
    let command = args.command;

    match command {
        _ => unimplemented!(),
    }
}
