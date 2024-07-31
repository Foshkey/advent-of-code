use std::{
    borrow::BorrowMut,
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use super::{module::Module, module_type::ModuleType, pulse::Pulse};
use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Configuration {
    state: HashMap<String, Module>,
    pulses: HashMap<Pulse, u64>,
}

impl Configuration {
    pub fn push_button(&mut self) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back((Pulse::Low, "broadcaster".to_string(), "button".to_string()));

        while let Some((pulse, name, from)) = queue.pop_front() {
            // First check if this is a low pulse to rx
            if pulse == Pulse::Low && name == "rx" {
                return true;
            }

            // Record pulse
            *self.pulses.entry(pulse).or_insert(0) += 1;

            // Find module
            let Some(module) = self.state.get_mut(&name) else {
                continue;
            };

            // Figure out the new pulse, if any
            let mut new_pulse = None;
            match module.module_type.borrow_mut() {
                ModuleType::Broadcaster => new_pulse = Some(pulse),
                ModuleType::FlipFlop(on) if pulse != Pulse::High => {
                    // If on, send low. If off, send high.
                    new_pulse = if *on {
                        Some(Pulse::Low)
                    } else {
                        Some(Pulse::High)
                    };
                    // Flip the flip-flop
                    *on = !*on;
                }
                ModuleType::Conjuction(memory) => {
                    // Update memory of the pulse
                    if let Some(memory_pulse) = memory.get_mut(&from) {
                        *memory_pulse = pulse
                    }

                    // Send low pulse if all are high, otherwise low
                    new_pulse = if memory.values().all(|&pulse| pulse == Pulse::High) {
                        Some(Pulse::Low)
                    } else {
                        Some(Pulse::High)
                    }
                }
                _ => (),
            }

            // Queue up the pulse to the destinations
            if let Some(new_pulse) = new_pulse {
                for destination in &module.outputs {
                    queue.push_back((new_pulse, destination.clone(), name.clone()));
                }
            }
        }

        false
    }

    pub fn get_total_pulses(&self) -> u64 {
        self.pulses[&Pulse::Low] * self.pulses[&Pulse::High]
    }
}

impl FromStr for Configuration {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Initial parse for the state
        let mut state = s
            .lines()
            .map(|l| -> Result<(String, Module)> {
                let module: Module = l.parse()?;
                Ok((module.name.clone(), module))
            })
            .collect::<Result<HashMap<String, Module>>>()?;

        // Wire up inputs for Conjuction modules
        for (name, module) in state.clone() {
            for output in module.outputs {
                let Some(output_module) = state.get_mut(&output) else {
                    continue;
                };

                if let ModuleType::Conjuction(memory) = output_module.module_type.borrow_mut() {
                    memory.insert(name.clone(), Pulse::Low);
                }
            }
        }

        // Initialize pulses
        let pulses = [(Pulse::Low, 0), (Pulse::High, 0)]
            .into_iter()
            .collect::<HashMap<Pulse, u64>>();

        Ok(Configuration { pulses, state })
    }
}

#[cfg(test)]
mod tests {
    use crate::models::module_type::ModuleType;

    use super::*;

    #[test]
    fn test_from_str() {
        let input = "broadcaster -> a, b, c\n%a -> b\n%b -> c\n%c -> inv\n&inv -> a";
        let config: Configuration = input.parse().unwrap();

        assert_eq!(config.state.len(), 5);
        let broadcaster = config.state.get("broadcaster").unwrap();
        let a = config.state.get("a").unwrap();
        let b = config.state.get("b").unwrap();
        let c = config.state.get("c").unwrap();
        let inv = config.state.get("inv").unwrap();

        // Test the types
        assert_eq!(broadcaster.module_type, ModuleType::Broadcaster);
        assert_eq!(a.module_type, ModuleType::FlipFlop(false));
        assert_eq!(b.module_type, ModuleType::FlipFlop(false));
        assert_eq!(c.module_type, ModuleType::FlipFlop(false));
        assert_eq!(
            inv.module_type,
            ModuleType::Conjuction(
                vec![("c".to_string(), Pulse::Low)]
                    .into_iter()
                    .collect::<HashMap<String, Pulse>>()
            )
        );

        // Test the outputs
        assert_eq!(broadcaster.outputs, vec!["a", "b", "c"]);
        assert_eq!(a.outputs, vec!["b"]);
        assert_eq!(b.outputs, vec!["c"]);
        assert_eq!(c.outputs, vec!["inv"]);
        assert_eq!(inv.outputs, vec!["a"]);
    }
}
