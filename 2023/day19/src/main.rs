use anyhow::Result;
use engines::{analyzer, runner};
use models::sequence::Sequence;

mod engines;
mod models;

fn part_1(input: &str) -> Result<u64> {
    let sequence: Sequence = input.parse()?;
    runner::run(&sequence)
}

fn part_2(input: &str) -> Result<u64> {
    let sequence: Sequence = input.parse()?;
    analyzer::count_distinct_combinations(&sequence)
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
        assert_eq!(19114, result.unwrap());
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("example.txt");
        let result = part_2(input);
        assert_eq!(167409079868000, result.unwrap());
    }
}
