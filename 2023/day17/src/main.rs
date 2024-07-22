use std::env;

use anyhow::Result;

use map::Map;
use path_finder::find_minimal_heatloss;

mod coord;
mod map;
mod path_finder;

fn part_1(input: &str) -> Result<u32> {
    let map: Map = input.parse()?;
    Ok(find_minimal_heatloss(&map, 0, 3))
}

fn part_2(input: &str) -> Result<u32> {
    let map: Map = input.parse()?;
    Ok(find_minimal_heatloss(&map, 4, 10))
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
        assert_eq!(102, result.unwrap());
    }

    #[test]
    fn test_part_2_1() {
        let input = include_str!("example.txt");
        let result = part_2(input);
        assert_eq!(94, result.unwrap());
    }

    #[test]
    fn test_part_2_2() {
        let input = include_str!("example2.txt");
        let result = part_2(input);
        assert_eq!(71, result.unwrap());
    }

    #[test]
    fn test_part_2_3() {
        let input = include_str!("example3.txt");
        let result = part_2(input);
        assert_eq!(34, result.unwrap());
    }
}
