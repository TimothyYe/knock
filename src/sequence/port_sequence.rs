use std::collections::{HashMap, HashSet};

use crate::config::Config;
use crate::sequence::SequenceDetector;

#[derive(Debug)]
pub struct PortSequenceDetector {
    timeout: i32,
    sequence_set: HashSet<i32>,
    sequence_rules: Vec<Vec<i32>>,
    client_sequences: HashMap<String, Vec<i32>>,
}

impl PortSequenceDetector {
    pub fn new(config: Config) -> PortSequenceDetector {
        let mut sequence_rules = Vec::new();
        for rule in config.rules.clone() {
            sequence_rules.push(rule.sequence);
        }

        let mut sequence_set = HashSet::new();
        for rule in config.rules {
            for sequence in rule.sequence {
                sequence_set.insert(sequence);
            }
        }

        PortSequenceDetector {
            timeout: config.timeout,
            sequence_set,
            sequence_rules,
            client_sequences: HashMap::new(),
        }
    }
}

impl SequenceDetector for PortSequenceDetector {
    fn add_sequence(&mut self, client_ip: String, sequence: i32) {
        // check if the sequence is in the set
        if !self.sequence_set.contains(&sequence) {
            return;
        }

        let client_sequence = self.client_sequences.entry(client_ip).or_insert(Vec::new());
        client_sequence.push(sequence);
    }

    fn match_sequence(&mut self, client_ip: &str) -> bool {
        // Check if the current sequence matches any of the rules
        let client_sequence = self.client_sequences.get_mut(client_ip);
        if let Some(sequence) = client_sequence {
            for rule in &self.sequence_rules {
                if sequence.ends_with(rule) {
                    println!("Matched sequence: {:?}", rule);
                    // clear the sequence
                    sequence.clear();
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_config() -> Config {
        Config {
            interface: "enp3s0".to_string(),
            timeout: 5,
            rules: vec![
                crate::config::Rule {
                    name: "enable ssh".to_string(),
                    sequence: vec![1, 2, 3],
                    command: "ls -lh".to_string(),
                },
                crate::config::Rule {
                    name: "disable ssh".to_string(),
                    sequence: vec![3, 5, 6],
                    command: "du -sh *".to_string(),
                },
            ],
        }
    }

    #[test]
    fn test_new() {
        let config = create_config();
        let detector = PortSequenceDetector::new(config);
        assert_eq!(detector.sequence_set.len(), 5);
        assert_eq!(detector.sequence_rules.len(), 2);
    }

    #[test]
    fn test_add_sequence() {
        let config = create_config();
        let mut detector = PortSequenceDetector::new(config);
        detector.add_sequence("127.0.0.1".to_owned(), 3);
        assert_eq!(detector.client_sequences.get("127.0.0.1"), Some(&vec![3]));
    }

    #[test]
    fn test_add_none_existing_sequence() {
        let config = create_config();
        let mut detector = PortSequenceDetector::new(config);
        detector.add_sequence("127.0.0.1".to_owned(), 9);
        assert_eq!(detector.client_sequences.get("127.0.0.1"), None);
    }

    #[test]
    fn test_match_sequence() {
        let config = create_config();
        let mut detector = PortSequenceDetector::new(config);
        detector.add_sequence("127.0.0.1".to_owned(), 1);
        detector.add_sequence("127.0.0.1".to_owned(), 3);
        detector.add_sequence("127.0.0.1".to_owned(), 5);
        detector.add_sequence("127.0.0.1".to_owned(), 6);
        assert_eq!(detector.match_sequence("127.0.0.1"), true);
        assert_eq!(detector.client_sequences.get("127.0.0.1").unwrap().len(), 0);
    }
}
