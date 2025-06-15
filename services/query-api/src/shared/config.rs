use serde::Deserialize;

pub const MAX_HEADER_WIDTH: usize = 10;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub query_server: String,
    pub graphql_server: String,
}
