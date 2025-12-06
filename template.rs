fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    input.len()
}

fn part_2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 0);
    }
}
