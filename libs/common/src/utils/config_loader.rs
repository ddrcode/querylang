use serde::de::DeserializeOwned;
use std::{env, path::PathBuf};

pub fn load_config<T: DeserializeOwned>(path: &str) -> Result<T, config::ConfigError> {
    let path = PathBuf::from(path).join("config");
    let env = env::var("APP_ENV").unwrap_or_else(|_| "development".into());
    let builder = config::Config::builder()
        .add_source(config::File::with_name(&join(&path, "default")))
        .add_source(config::File::with_name(&join(&path, &env)).required(false))
        .add_source(config::Environment::default().separator("__"));
    dotenv::dotenv().ok();
    builder.build()?.try_deserialize()
}

fn join(base: &PathBuf, file: &str) -> String {
    base.join(file)
        .to_str()
        .expect("Path is not valid UTF-8")
        .into()
}
