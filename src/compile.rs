use crate::init::check_vault;
use crate::magic::COMPILE_SCRATCH_PATH;
use anyhow::{anyhow, bail, Context, Result};
use std::fs::read_dir;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Component;
use std::path::PathBuf;
use std::process::Command;

pub fn command(output_path: &str, open_command: Option<&str>) -> Result<()> {
    check_vault().context("Not in a valid vault.")?;

    let directory = read_dir(".").context("Couldn't get contents of current directory.")?;
    let files = directory
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .map(|ext| ext.eq_ignore_ascii_case("typ"))
                .unwrap_or(false)
        });

    let mut relative = PathBuf::new();
    for component in PathBuf::from(COMPILE_SCRATCH_PATH).components().skip(1) {
        match component {
            Component::Normal(_) => relative.push(Component::ParentDir),
            _ => bail!(format!(
                "Found unexpected component {component:?} in \
                working directory path {COMPILE_SCRATCH_PATH:?}.",
            )),
        }
    }

    let scratch = File::create(COMPILE_SCRATCH_PATH).context(format!(
        "Failed to create scratch file {COMPILE_SCRATCH_PATH:?}."
    ))?;
    let mut writer = BufWriter::new(scratch);
    for file in files {
        let path = relative.join(file);
        writeln!(writer, r#"#include("{}")"#, path.to_string_lossy())
            .context(format!("Failed to write to {COMPILE_SCRATCH_PATH:?}"))?;
    }
    writer.flush().context(format!(
        "Failed to flush writer for scratch file {COMPILE_SCRATCH_PATH:?}."
    ))?;

    let command_output = Command::new("typst")
        .args(["compile", COMPILE_SCRATCH_PATH, output_path, "--root", "."])
        .args(
            open_command
                .map(|program| vec!["--open", program])
                .unwrap_or_default(),
        )
        .output()
        .context("Failed to run typst compile.")?;
    if !command_output.stderr.is_empty() {
        return Err(anyhow!(
            String::from_utf8_lossy(&command_output.stderr).to_string()
        ))
        .context(format!(
            "Typst compile failed for input {COMPILE_SCRATCH_PATH:?} \
            and output {output_path:?}."
        ));
    }

    Ok(())
}
