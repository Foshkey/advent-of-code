use network::Network;

mod network;

const INPUT: &str = include_str!("../input.txt");

fn part_1(input: &str) -> usize {
    let network: Network = input.into();
    network
        .get_tri_connections(|s: &str| s.starts_with('t'))
        .len()
}

fn part_2(input: &str) -> usize {
    input.len()
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
        let example = include_str!("../example.txt");
        assert_eq!(part_1(example), 7);
    }
}
