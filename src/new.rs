use crate::config::get_config;
use crate::init::check_vault;
use anyhow::{anyhow, bail, Context, Result};
use chrono::prelude::*;
use rand::seq::SliceRandom;
use std::fmt::Debug;
use std::fs::File;
use std::io::{Read, Write};
use std::os::unix::process::CommandExt;
use std::process::Command;

fn make_proquint() -> String {
    let consonants = vec![
        'b', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'r', 's', 't', 'v', 'z',
    ];
    let vowels = vec!['a', 'i', 'o', 'u'];
    let mut rng = rand::thread_rng();

    [&consonants, &vowels]
        .iter()
        .cycle()
        .take(5)
        .map(|arr| *arr.choose(&mut rng).unwrap())
        .collect()
}

pub fn command(name: Option<&str>, open: Option<&str>) -> Result<()> {
    check_vault().context("Not in a valid vault.")?;

    let config = get_config();

    let filename = match (config.get_string("new.name"), name) {
        (_, Some(name)) => name.to_owned(),
        (Ok(name), None) => name,
        (Err(_), None) => {
            bail!(
                "No file name was provided for new note,\
                 and no default file name is set in config."
            );
        }
    };

    let uuid = (1..=3)
        .map(|_| make_proquint())
        .collect::<Vec<_>>()
        .join("-");
    let filename = format!(
        "{}.typ",
        if filename == "proquint" {
            &uuid
        } else {
            &filename
        }
    );
    let now: DateTime<Local> = Local::now();
    let mut file = File::create_new(&filename).context("Failed to create new note.")?;
    file.write_all(
        format!(
            r##"#import "libraries/basalt.typ": note,
#note(
  "{uuid}",
  created: datetime(year: {}, month: {}, day: {}, hour: {}, minute: {}, second: {}),
)

"##,
            now.year(),
            now.month(),
            now.day(),
            now.hour(),
            now.minute(),
            now.second(),
        )
        .as_bytes(),
    )
    .context("Failed to write to new note.")?;

    if let Some(open) = open {
        Command::new(open).arg(filename).exec();
        // .context("Unable to run open command.")?;
    }

    Ok(())
}
