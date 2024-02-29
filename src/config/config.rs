extern crate serde;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    pub name: String,
    pub sequence: Vec<i32>,
    pub timeout: i32,
    pub command: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub interface: String,
    pub rules: Vec<Rule>,
}
