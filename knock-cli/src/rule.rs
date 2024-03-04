use crate::config::Config;
use crate::config::Rule;
use log::{error, info};
use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;

pub struct RuleExecutor {
    rules: HashMap<String, Rule>,
}

impl RuleExecutor {
    #[must_use]
    pub fn new(config: Config) -> RuleExecutor {
        let mut rules = HashMap::new();
        for rule in config.rules {
            rules.insert(rule.name.clone(), rule);
        }

        RuleExecutor { rules }
    }

    pub fn run(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(rule) = self.rules.get(name) {
            info!("Executing rule: {}", rule.name);
            // Iterate over the ports and attempt to connect to each
            for port in rule.sequence.iter() {
                let address = format!("{}:{}", rule.host, port);
                let addr: Vec<SocketAddr> = address.to_socket_addrs()?.collect();
                info!("Knocking at: {}", addr[0]);

                // Attempt to connect to the target IP and port
                if let Ok(stream) = TcpStream::connect_timeout(&addr[0], Duration::from_millis(100))
                {
                    drop(stream);
                }
            }
        } else {
            error!("Rule not found: {}", name);
            return Ok(());
        }

        info!("Rule execution complete.");
        Ok(())
    }
}
