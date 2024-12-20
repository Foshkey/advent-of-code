use std::collections::{HashMap, HashSet};

pub struct PatternAssembler {
    towels: Vec<String>,
    possibilities: HashMap<String, usize>,
    not_possible: HashSet<String>,
}

impl PatternAssembler {
    pub fn is_possible(&mut self, pattern: String) -> bool {
        if pattern.is_empty() || self.possibilities.contains_key(&pattern) {
            return true;
        }

        if self.not_possible.contains(&pattern) {
            return false;
        }

        for towel in self.towels.clone() {
            if pattern.starts_with(&towel) && self.is_possible(pattern[towel.len()..].to_string()) {
                self.possibilities.insert(pattern, 1);
                return true;
            }
        }

        self.not_possible.insert(pattern);
        false
    }

    pub fn count_possibilities(&mut self, pattern: String) -> usize {
        if pattern.is_empty() {
            return 1;
        }

        if self.not_possible.contains(&pattern) {
            return 0;
        }

        if let Some(&count) = self.possibilities.get(&pattern) {
            return count;
        }

        for towel in self.towels.clone() {
            if !pattern.starts_with(&towel) {
                continue;
            }

            let count = self.count_possibilities(pattern[towel.len()..].to_string());
            if count == 0 {
                continue;
            }

            *self.possibilities.entry(pattern.clone()).or_default() += count;
        }

        if let Some(&count) = self.possibilities.get(&pattern) {
            return count;
        }

        self.not_possible.insert(pattern);

        0
    }
}

impl From<&str> for PatternAssembler {
    fn from(s: &str) -> PatternAssembler {
        PatternAssembler {
            towels: s.split(", ").map(|s| s.to_string()).collect(),
            possibilities: HashMap::new(),
            not_possible: HashSet::new(),
        }
    }
}
