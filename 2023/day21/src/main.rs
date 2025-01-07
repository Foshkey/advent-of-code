use map::Map;

mod map;

fn get_num_spaces(input: &str, distance: usize) -> usize {
    let map: Map = input.parse().unwrap();
    map.get_num_spaces(distance)
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", get_num_spaces(input, 64));
    //println!("Part 2: {}", get_num_spaces(input, 26501365));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let example = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!(get_num_spaces(example, 6), 16);
        assert_eq!(get_num_spaces(example, 10), 50);
        // assert_eq!(get_num_spaces(example, 50), 1594);
        // assert_eq!(get_num_spaces(example, 100), 6536);
        // assert_eq!(get_num_spaces(example, 500), 167004);
        // assert_eq!(get_num_spaces(example, 1000), 668697);
        // assert_eq!(get_num_spaces(example, 5000), 16733044);
    }
}
