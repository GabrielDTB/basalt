use crate::magic::BASE_PATH;
use anyhow::Result;
use config::{Config, File};
use const_format::formatcp;

pub fn get_config() -> Result<()> {
    let settings = Config::builder()
        .add_source(File::with_name(formatcp!("{BASE_PATH}basalt.toml")))
        .build()
        .unwrap();

    println!(
        "{:?}",
        settings // .try_deserialize::<HashMap<String, String>>()
                 // .unwrap()
    );
    Ok(())
}
