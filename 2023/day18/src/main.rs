use anyhow::Result;

use instruction::Instruction;
use map::Map;

mod direction;
mod instruction;
mod map;
mod point;
mod rectangle;

fn part_1(input: &str) -> Result<u64> {
    let instructions = Instruction::parse_set(input)?;
    let map = Map::new(instructions);
    Ok(map.count_filled())
}

fn part_2(input: &str) -> Result<u64> {
    let instructions = Instruction::parse_set_from_color(input)?;
    let map = Map::new(instructions);
    Ok(map.count_filled())
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {:?}", part_1(input));
    println!("Part 2: {:?}", part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("example.txt");
        let result = part_1(input);
        assert_eq!(62, result.unwrap());
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("example.txt");
        let result = part_2(input);
        assert_eq!(952408144115, result.unwrap());
    }
}
