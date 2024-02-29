mod config;

use config::Config;
use std::fs::File;
use std::io::Read;

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open("config.yaml")?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;
    let config: Config = serde_yaml::from_str(&content)?;

    Ok(config)
}

// test case for load_config
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let config = load_config().unwrap();
        assert_eq!(config.interface, "enp3s0");
        assert_eq!(config.rules.len(), 2);
    }
}
