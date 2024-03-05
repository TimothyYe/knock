use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub name: String,
    pub sequence: Vec<i32>,
    pub command: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub interface: String,
    pub timeout: u64,
    pub rules: Vec<Rule>,
}
