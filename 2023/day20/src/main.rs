use anyhow::Result;
use models::configuration::Configuration;

mod models;

fn part_1(input: &str) -> Result<u64> {
    let mut config: Configuration = input.parse()?;

    // lol button go brrrr
    for _ in 0..1000 {
        config.push_button();
    }

    Ok(config.get_total_pulses())
}

fn part_2(input: &str) -> Result<u64> {
    let mut config: Configuration = input.parse()?;

    let mut count = 0;
    loop {
        count += 1;
        if config.push_button() {
            break;
        }
    }

    Ok(count)
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
    fn test_part_1_1() {
        let input = include_str!("example1.txt");
        let result = part_1(input);
        assert_eq!(32000000, result.unwrap());
    }

    #[test]
    fn test_part_1_2() {
        let input = include_str!("example2.txt");
        let result = part_1(input);
        assert_eq!(11687500, result.unwrap());
    }
}
