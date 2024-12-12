use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

struct Rules {
    // This is a hashmap indicating all the numbers that the key has to be before in an update.
    rules: HashMap<usize, HashSet<usize>>,
}

impl Rules {
    fn get_offender(&self, update: &[usize]) -> Option<usize> {
        let mut prev_nums = Vec::<usize>::new();

        for (i, num) in update.iter().enumerate() {
            // Check if there's a rule for this number
            if let Some(rule) = self.rules.get(num) {
                // And then check if it has occurred after any of the numbers in the rule.
                if prev_nums.iter().any(|n| rule.contains(n)) {
                    // If so, it's out of place.
                    return Some(i);
                }
            };

            prev_nums.push(*num);
        }

        // If we went though all of them without returning, then there's no offender. Update is correct.
        None
    }

    fn get_correct_order(&self, update: &[usize]) -> Vec<usize> {
        let mut new_update: Vec<usize> = update.to_vec();

        // Keep looping while we have offenders.
        while let Some(index) = self.get_offender(&new_update) {
            // Rip out the offender.
            let num = new_update.remove(index);
            // Get the rule. We shouldn't get None but break in case.
            let Some(rule) = self.rules.get(&num) else {
                break;
            };

            // Loop until we find a number it has to be before.
            for (i, n) in new_update.iter().enumerate() {
                if rule.contains(n) {
                    // Insert it just before that number.
                    new_update.insert(i, num);
                    break;
                }
            }
        }

        new_update
    }
}

impl From<&str> for Rules {
    fn from(value: &str) -> Self {
        let mut rules = HashMap::<usize, HashSet<usize>>::new();

        for line in value.lines() {
            let Some((x_str, y_str)) = line.split_once('|') else {
                continue;
            };

            let x = x_str.parse::<usize>().unwrap();
            let y = y_str.parse::<usize>().unwrap();

            if let Some(set) = rules.get_mut(&x) {
                set.insert(y);
            } else {
                rules.insert(x, HashSet::from_iter(vec![y]));
            }
        }

        Rules { rules }
    }
}

fn parse_input(input: &str) -> (Rules, Vec<Vec<usize>>) {
    let Some((rules_str, updates_str)) = input.split_once("\n\n") else {
        panic!("Could not find double line break.");
    };

    let rules = rules_str.into();
    let updates = updates_str
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    (rules, updates)
}

fn part_1(input: &str) -> usize {
    let (rules, updates) = parse_input(input);

    updates
        .iter()
        .filter(|&update| rules.get_offender(update).is_none())
        .map(|update| update[update.len() / 2])
        .sum()
}

fn part_2(input: &str) -> usize {
    let (rules, updates) = parse_input(input);

    updates
        .iter()
        .filter(|&update| rules.get_offender(update).is_some())
        .map(|update| rules.get_correct_order(update))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn main() {
    println!("Part 1: {:?}", part_1(INPUT));
    println!("Part 2: {:?}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn test_example_part_1() {
        let result = part_1(EXAMPLE);
        assert_eq!(result, 143);
    }

    #[test]
    fn test_example_part_2() {
        let result = part_2(EXAMPLE);
        assert_eq!(result, 123);
    }
}
