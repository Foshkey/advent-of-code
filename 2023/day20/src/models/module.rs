use std::{collections::HashMap, str::FromStr};

use anyhow::bail;

use super::module_type::ModuleType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub module_type: ModuleType,
    pub name: String,
    pub outputs: Vec<String>,
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
