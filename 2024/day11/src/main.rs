mod stone_line;

const INPUT: &str = include_str!("input.txt");

fn part_1(input: &str) -> usize {
    let mut stone_line: stone_line::StoneLine = input.into();
    for _ in 0..25 {
        stone_line.blink();
    }
    stone_line.len()
}

fn part_2(input: &str) -> usize {
    let mut stone_line: stone_line::StoneLine = input.into();
    for _ in 0..75 {
        stone_line.blink();
    }
    stone_line.len()
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
        assert_eq!(result, 55312);
    }
}
