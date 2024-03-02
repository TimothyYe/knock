use crate::config::Config;
use crate::config::Rule;
use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;

pub struct RuleExecutor {
    rules: HashMap<String, Rule>,
}

impl RuleExecutor {
    pub fn new(config: Config) -> RuleExecutor {
        let mut rules = HashMap::new();
        for rule in config.rules {
            rules.insert(rule.name.clone(), rule);
        }

        RuleExecutor { rules }
    }

    pub fn run(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(rule) = self.rules.get(name) {
            println!("Executing rule: {}", rule.name);
            // Iterate over the ports and attempt to connect to each
            for port in rule.sequence.iter() {
                let address = format!("{}:{}", rule.host, port);
                let addr: Vec<SocketAddr> = address.to_socket_addrs()?.collect();
                println!("knocking at: {:?}", addr);

                // Attempt to connect to the target IP and port
                if let Ok(stream) = TcpStream::connect_timeout(&addr[0], Duration::from_millis(100))
                {
                    drop(stream);
                }
            }
        } else {
            println!("Rule not found: {}", name);
            return Ok(());
        }

        println!("Rule execution complete.");
        Ok(())
    }
}
