use schematic::Schematic;

mod schematic;

const INPUT: &str = include_str!("../input.txt");

fn part_1(input: &str) -> usize {
    let schematic: Schematic = input.parse().unwrap();
    schematic.get_unique_fits()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let example = include_str!("../example.txt");
        assert_eq!(part_1(example), 3);
    }
}
