use std::collections::HashMap;

use super::pulse::Pulse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleType {
    Broadcaster,
    Conjuction(HashMap<String, Pulse>),
    FlipFlop(bool),
}
