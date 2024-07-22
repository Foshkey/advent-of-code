use anyhow::{bail, Result};

use crate::direction::Direction;

pub struct Instruction {
    pub direction: Direction,
    pub length: u32,
    pub color: u32,
}

pub type Instructions = Vec<Instruction>;

impl Instruction {
    pub fn new(input: &str) -> Result<Self> {
        let parts: Vec<&str> = input.trim().splitn(3, ' ').collect();
        if parts.len() != 3 {
            bail!("Invalid instruction format: {}", input);
        }

        let direction = Direction::new(match parts[0].chars().next() {
            Some(c) => c,
            None => bail!("Invalid direction: {}", parts[0]),
        })?;

        let length = match parts[1].parse() {
            Ok(value) => value,
            Err(error) => bail!("Invalid length {}. {}", parts[1], error),
        };

        let color_str = parts[2].trim_matches(|c| c == '(' || c == ')');
        let color = match u32::from_str_radix(&color_str[1..], 16) {
            Ok(value) => value,
            Err(error) => bail!("Invalid color: {}. {}", color_str, error),
        };

        Ok(Instruction {
            direction,
            length,
            color,
        })
    }

    pub fn parse_set(input: &str) -> Result<Instructions> {
        input.lines().map(Instruction::new).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_new_valid_input() {
        let input = "R 10 (#FF0000)";
        let instruction = Instruction::new(input).unwrap();

        assert_eq!(instruction.direction, Direction { x: 1, y: 0 });
        assert_eq!(instruction.length, 10);
        assert_eq!(instruction.color, 0xFF0000);
    }
}
