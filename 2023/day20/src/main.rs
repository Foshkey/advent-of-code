use configuration::Configuration;

mod configuration;
mod math;
mod module;

fn part_1(input: &str) -> usize {
    let mut config: Configuration = input.parse().unwrap();

    for _ in 0..1000 {
        config.push_button();
    }

    config.get_total_pulses()
}

fn part_2(input: &str) -> usize {
    let mut config: Configuration = input.parse().unwrap();
    config.get_min_presses_rx()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {:?}", part_1(input));
    println!("Part 2: {:?}", part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_1() {
        let input = include_str!("../example1.txt");
        let result = part_1(input);
        assert_eq!(32000000, result);
    }

    #[test]
    fn test_part_1_2() {
        let input = include_str!("../example2.txt");
        let result = part_1(input);
        assert_eq!(11687500, result);
    }
}
