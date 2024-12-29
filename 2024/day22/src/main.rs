mod number_generator;
mod sequence_finder;

const INPUT: &str = include_str!("../input.txt");

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|l| number_generator::generate_number(l.parse().unwrap(), 2000))
        .sum()
}

fn part_2(input: &str) -> usize {
    sequence_finder::find_max_bananas(input, 2000)
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let example = "\
1
10
100
2024";
        assert_eq!(part_1(example), 37327623);
    }

    #[test]
    fn test_example_2() {
        let example = "\
1
2
3
2024";
        assert_eq!(part_2(example), 23);
    }
}
