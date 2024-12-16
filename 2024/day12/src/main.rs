mod coord;
mod map;
mod region;

const INPUT: &str = include_str!("../input.txt");

fn part_1(input: &str) -> usize {
    let map: map::Map = input.into();
    map.regions()
        .iter()
        .map(|region| region.perimeter() * region.len())
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

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_example_part_1() {
        let result = part_1(EXAMPLE);
        assert_eq!(result, 1930);
    }
}
