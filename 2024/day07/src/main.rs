mod equation;

const INPUT: &str = include_str!("input.txt");

fn part_1(input: &str) -> u128 {
    input
        .lines()
        .filter_map(|line| line.parse::<equation::Equation>().ok())
        .filter(|eq| eq.is_possible())
        .map(|eq| eq.result)
        .sum()
}

fn part_2(input: &str) -> usize {
    input.len()
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
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_example_part_2() {
        let result = part_2(EXAMPLE);
        assert_eq!(result, 11387);
    }
}
