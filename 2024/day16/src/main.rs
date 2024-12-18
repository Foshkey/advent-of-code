use maze::Maze;

mod maze;

const INPUT: &str = include_str!("../input.txt");

fn part_1(input: &str) -> usize {
    let maze: Maze = input.into();
    maze.get_best_paths(false).unwrap().score
}

fn part_2(input: &str) -> usize {
    let maze: Maze = input.into();
    maze.get_best_paths(true).unwrap().best_tiles.len()
}

fn main() {
    println!("Part 1: {:?}", part_1(INPUT));
    println!("Part 2: {:?}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let example = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!(part_1(example), 7036);
        assert_eq!(part_2(example), 45);
    }

    #[test]
    fn test_example_2() {
        let example = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        assert_eq!(part_1(example), 11048);
        assert_eq!(part_2(example), 64);
    }
}
