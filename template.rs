fn part_1(input: &str) -> usize {
    input.len()
}

fn part_2(input: &str) -> usize {
    input.len()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let example = "";
        assert_eq!(part_1(example), 0);
        assert_eq!(part_2(example), 0);
    }
}
