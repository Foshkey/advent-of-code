use maze::Maze;

mod maze;

const INPUT: &str = include_str!("../input.txt");

fn part_1(input: &str, threshold: usize) -> usize {
    let maze: Maze = input.parse().unwrap();
    let cheats = maze.find_cheats();
    cheats
        .iter()
        .filter(|&&savings| savings >= threshold)
        .count()
}

fn part_2(input: &str) -> usize {
    input.len()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT, 100));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let example = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

        assert_eq!(part_1(example, 0), 30);
    }
}
