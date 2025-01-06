use std::{
    borrow::BorrowMut,
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use anyhow::Result;

use crate::module::{Module, ModuleType, Pulse, PulseResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Configuration {
    state: HashMap<String, Module>,
    pulses: HashMap<Pulse, usize>,
}

impl Configuration {
    pub fn push_button(&mut self) {
        let mut queue = VecDeque::new();
        queue.push_back(PulseResult {
            pulse: Pulse::Low,
            destination: "broadcaster".to_string(),
            from: "button".to_string(),
        });

        while let Some(PulseResult {
            pulse,
            destination: name,
            from,
        }) = queue.pop_front()
        {
            // Record pulse
            *self.pulses.entry(pulse).or_insert(0) += 1;

            // Find module
            let Some(module) = self.state.get_mut(&name) else {
                continue;
            };

            // Process pulses
            let results = module.process_pulse(pulse, &from);

            // Add the results to the queue
            queue.extend(results);
        }
    }

    pub fn get_total_pulses(&self) -> usize {
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
            .collect::<HashMap<Pulse, usize>>();

        Ok(Configuration { pulses, state })
    }
}

#[cfg(test)]
mod tests {
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
