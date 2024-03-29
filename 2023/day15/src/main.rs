use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, bail, Error, Result};

enum Op {
    AddLens(Lense),
    RemoveLens(String),
}

impl FromStr for Op {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('=') {
            let (label, focal_length_str) = s
                .split_once('=')
                .ok_or(anyhow!("Could not split at '=' in '{s}'."))?;
            let focal_length = focal_length_str.parse()?;
            return Ok(Op::AddLens(Lense {
                label: label.to_string(),
                focal_length,
            }));
        }

        if s.contains('-') {
            let (label, _) = s
                .split_once('-')
                .ok_or(anyhow!("Could not split at '-' in '{s}'."))?;
            return Ok(Op::RemoveLens(label.to_string()));
        }

        bail!("Step does not contain valid operations of '=' or '-'")
    }
}

#[derive(Debug)]
struct Lense {
    label: String,
    focal_length: u8,
}

#[derive(Debug)]
struct Box {
    lenses: Vec<Lense>,
}

#[derive(Debug)]
struct BoxLine {
    boxes: HashMap<u8, Box>,
}

impl BoxLine {
    fn process_sequence(s: &str) -> Result<Self> {
        let mut box_line = BoxLine {
            boxes: HashMap::new(),
        };

        for step in strip_newlines(s).split(',') {
            let op = step.parse()?;
            box_line.execute(op);
        }

        Ok(box_line)
    }

    fn execute(&mut self, op: Op) {
        match op {
            Op::AddLens(lense) => self.add_lense(lense),
            Op::RemoveLens(label) => self.remove_lense(label),
        }
    }

    fn add_lense(&mut self, lense: Lense) {
        let box_index = hash(&lense.label);

        // Check if box exists
        if let Some(b) = self.boxes.get_mut(&box_index) {
            // If so, check if there's already a lense with the same label
            if let Some(lense_index) = b.lenses.iter().position(|l| l.label == lense.label) {
                // Replace the lense at the given index
                b.lenses[lense_index] = lense;
            } else {
                // Add the lense to the end
                b.lenses.push(lense);
            }
        } else {
            // Initialize a new box with the lense
            self.boxes.insert(
                box_index,
                Box {
                    lenses: vec![lense],
                },
            );
        }
    }

    fn remove_lense(&mut self, label: String) {
        let box_index = hash(&label);

        // Check if box exists
        if let Some(b) = self.boxes.get_mut(&box_index) {
            // Look for the lense with the same label
            if let Some(lense_index) = b.lenses.iter().position(|l| l.label == label) {
                // Remove it
                b.lenses.remove(lense_index);
            }
        }
    }

    fn get_total_focusing_power(&self) -> usize {
        self.boxes
            .iter()
            .map(|(&b_i, b)| {
                b.lenses
                    .iter()
                    .enumerate()
                    .map(|(l_i, l)| (b_i as usize + 1) * (l_i + 1) * l.focal_length as usize)
                    .sum::<usize>()
            })
            .sum()
    }
}

fn hash(input: &str) -> u8 {
    let mut current_value = 0;
    for c in input.chars() {
        let mut temp = current_value as u16;
        temp += c as u16;
        temp *= 17;
        current_value = temp as u8;
    }
    current_value
}

fn strip_newlines(input: &str) -> String {
    input.replace(&['\n', '\r'][..], "")
}

fn get_verification_number(input: &str) -> usize {
    strip_newlines(input)
        .split(',')
        .map(|s| hash(s) as usize)
        .sum()
}

fn part_2(input: &str) -> Result<usize> {
    let box_line = BoxLine::process_sequence(input)?;
    Ok(box_line.get_total_focusing_power())
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", get_verification_number(input));
    println!("Part 2: {:?}", part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(52, hash("HASH"))
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("example.txt");
        let result = get_verification_number(input);
        assert_eq!(1320, result);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("example.txt");
        let result = part_2(input);
        assert_eq!(145, result.unwrap());
    }
}
