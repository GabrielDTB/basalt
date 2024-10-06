use crate::magic::NEEDS_TO_EXIST;
use anyhow::{bail, Context, Result};
use std::fs::create_dir;
use std::fs::metadata;
use std::io::ErrorKind::AlreadyExists;

pub fn check_vault() -> Result<()> {
    for path in NEEDS_TO_EXIST {
        let md = metadata(path).context(format!("Could not get info about {path:?}."))?;
        if !md.is_dir() {
            bail!("{path:?} exists but is not a directory.");
        }
    }

    Ok(())
}

pub fn command() -> Result<()> {
    for path in NEEDS_TO_EXIST {
        if let Ok(md) = metadata(path) {
            if !md.is_dir() {
                bail!("{path:?} exists but is not a directory.");
            }
        }

        if let Err(e) = create_dir(path)
            && e.kind() != AlreadyExists
        {
            return Err(e).context(format!("Failed to create {path:?}."));
        };
    }

    Ok(())
}
