use std::{borrow::BorrowMut, collections::HashMap, str::FromStr};

use anyhow::bail;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub module_type: ModuleType,
    pub name: String,
    pub outputs: Vec<String>,
}

impl Module {
    pub fn process_pulse(&mut self, pulse: Pulse, from: &String) -> Vec<PulseResult> {
        let new_pulse = match self.module_type.borrow_mut() {
            ModuleType::Broadcaster => Some(pulse),
            ModuleType::FlipFlop(on) if pulse == Pulse::Low => {
                // If on, send low. If off, send high.
                let pulse = if *on { Pulse::Low } else { Pulse::High };
                // Flip the flip-flop
                *on = !*on;
                // Send pulse
                Some(pulse)
            }
            ModuleType::FlipFlop(_) => None,
            ModuleType::Conjuction(memory) => {
                // Update memory of the pulse
                if let Some(memory_pulse) = memory.get_mut(from) {
                    *memory_pulse = pulse
                }

                // Send low pulse if all are high, otherwise low
                if memory.values().all(|&pulse| pulse == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
        };

        // Queue up the pulse to the destinations
        let Some(new_pulse) = new_pulse else {
            return Vec::new();
        };

        self.outputs
            .iter()
            .map(|destination| PulseResult {
                pulse: new_pulse,
                destination: destination.clone(),
                from: self.name.clone(),
            })
            .collect()
    }
}

impl FromStr for Module {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("->").map(|s| s.trim()).collect();
        if parts.len() < 2 {
            bail!("Expected '->' separator within string: {}", s)
        }

        let module_type = match parts[0] {
            s if s.starts_with('%') => ModuleType::FlipFlop(false),
            s if s.starts_with('&') => ModuleType::Conjuction(HashMap::new()),
            _ => ModuleType::Broadcaster,
        };

        Ok(Module {
            module_type,
            name: parts[0].trim_matches(|c| c == '&' || c == '%').to_string(),
            outputs: parts[1].split(',').map(|s| s.trim().to_string()).collect(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleType {
    Broadcaster,
    Conjuction(HashMap<String, Pulse>),
    FlipFlop(bool),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PulseResult {
    pub pulse: Pulse,
    pub from: String,
    pub destination: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_broadcaster() {
        let input = "broadcaster -> a, b, c";
        let module: Module = input.parse().unwrap();

        assert_eq!(module.module_type, ModuleType::Broadcaster);
        assert_eq!(module.name, "broadcaster");
        assert_eq!(module.outputs, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_from_flip_flop() {
        let input = "%a -> b";
        let module: Module = input.parse().unwrap();

        assert_eq!(module.module_type, ModuleType::FlipFlop(false));
        assert_eq!(module.name, "a");
        assert_eq!(module.outputs, vec!["b"]);
    }

    #[test]
    fn test_from_conjuction() {
        let input = "&inv -> a";
        let module: Module = input.parse().unwrap();

        assert_eq!(module.module_type, ModuleType::Conjuction(HashMap::new()));
        assert_eq!(module.name, "inv");
        assert_eq!(module.outputs, vec!["a"]);
    }
}
