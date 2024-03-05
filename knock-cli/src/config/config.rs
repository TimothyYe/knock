use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub name: String,
    pub host: String,
    pub sequence: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub rules: Vec<Rule>,
}
