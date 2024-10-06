use const_format::formatcp;
use std::path::MAIN_SEPARATOR as SEP;

pub const BASE_PATH: &str = formatcp!(".basalt");
pub const OUTPUT_PATH: &str = formatcp!("{BASE_PATH}{SEP}output");
pub const WORKING_PATH: &str = formatcp!("{BASE_PATH}{SEP}working");
pub const NEEDS_TO_EXIST: [&str; 3] = [BASE_PATH, OUTPUT_PATH, WORKING_PATH];

pub const COMPILE_SCRATCH_PATH: &str = formatcp!("{WORKING_PATH}{SEP}vault.typ");
pub const DEFAULT_OUTPUT_PATH: &str = formatcp!("{OUTPUT_PATH}{SEP}vault.pdf");
