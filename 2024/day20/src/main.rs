use maze::Maze;

mod maze;

const INPUT: &str = include_str!("../input.txt");

fn part_1(input: &str, threshold: usize) -> usize {
    let maze: Maze = input.parse().unwrap();
    maze.find_paths(2, threshold)
}

fn part_2(input: &str, threshold: usize) -> usize {
    let maze: Maze = input.parse().unwrap();
    maze.find_paths(20, threshold)
}

fn main() {
    println!("Part 1: {}", part_1(INPUT, 100));
    println!("Part 2: {}", part_2(INPUT, 100));
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

        assert_eq!(part_1(example, 2), 44);
    }
}
