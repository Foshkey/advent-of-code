use std::io::Write;
use std::{env, fs::File};

use anyhow::Result;

use instruction::Instruction;
use map::Map;

mod direction;
mod instruction;
mod map;

fn part_1(input: &str) -> Result<u32> {
    let instructions = Instruction::parse_set(input)?;
    let mut map = Map::new(instructions)?;
    map.fill()?;

    // let mut file = File::create("output_filled.txt")?;
    // write!(file, "{}", map)?;

    Ok(map.count())
}

fn part_2(input: &str) -> Result<u32> {
    let instructions = Instruction::parse_set(input)?;
    Ok(0)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = include_str!("input.txt");

    let result = match args.get(1).map(|s| s.as_str()) {
        Some("1") => part_1(input),
        Some("2") => part_2(input),
        _ => part_1(input),
    };

    println!("{:?}", result);
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
}
