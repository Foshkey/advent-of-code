use crate::manifold::Manifold;

mod manifold;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let manifold: Manifold = input.into();
    manifold.get_num_splits()
}

fn part_2(input: &str) -> usize {
    let manifold: Manifold = input.into();
    manifold.get_num_timelines()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 21);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 40);
    }
}
