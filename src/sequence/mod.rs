pub use port_sequence::PortSequenceDetector;

mod port_sequence;

pub trait SequenceDetector {
    fn add_sequence(&mut self, client_ip: String, sequence: i32);
    fn match_sequence(&mut self, client_ip: &str) -> bool;
}
