use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub metrics_server: String
}
