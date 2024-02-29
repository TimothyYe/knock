extern crate serde;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    name: String,
    sequence: Vec<i32>,
    timeout: i32,
    command: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    interface: String,
    rules: Vec<Rule>,
}
