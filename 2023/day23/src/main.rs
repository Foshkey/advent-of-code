use map::Map;

mod map;

fn part_1(input: &str) -> usize {
    let map: Map = input.into();
    map.get_longest_path(true).len()
}

fn part_2(input: &str) -> usize {
    let map: Map = input.into();
    map.get_longest_path(false).len()
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
        let example = include_str!("../example.txt");
        assert_eq!(part_1(example), 94);
        assert_eq!(part_2(example), 154);
    }
}
