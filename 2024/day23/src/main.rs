use network::Network;

mod network;

const INPUT: &str = include_str!("../input.txt");

fn part_1(input: &str) -> usize {
    let network: Network = input.into();
    network
        .get_tri_connections(|s: &str| s.starts_with('t'))
        .len()
}

fn part_2(input: &str) -> String {
    let network: Network = input.into();
    let mut max_clique: Vec<_> = network.find_maximum_clique().into_iter().collect();
    max_clique.sort();
    max_clique.join(",")
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
        assert_eq!(part_2(example), "co,de,ka,ta");
    }
}
