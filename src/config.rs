use crate::magic::CONFIG_PATH;
use config::Config;
use config::File;

pub fn get_config() -> Config {
    let base = Config::builder();
    match base
        .clone()
        .add_source(File::with_name(CONFIG_PATH))
        .build()
    {
        Ok(config) => config,
        _ => base
            .build()
            .expect("Base config failed to build. The devs messed up."),
    }
}
