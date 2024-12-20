use pattern_assembler::PatternAssembler;

mod pattern_assembler;

const INPUT: &str = include_str!("../input.txt");

fn part_1(input: &str) -> usize {
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    let mut assembler: PatternAssembler = towels.into();

    patterns
        .lines()
        .filter(|&pattern| assembler.is_possible(pattern.to_string()))
        .count()
}

fn part_2(input: &str) -> usize {
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    let mut assembler: PatternAssembler = towels.into();

    patterns
        .lines()
        .map(|pattern| assembler.count_possibilities(pattern.to_string()))
        .sum()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let example = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!(part_1(example), 6);
        assert_eq!(part_2(example), 16);
    }
}
