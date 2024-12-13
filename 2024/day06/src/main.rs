mod map;
mod tracker;

const INPUT: &str = include_str!("input.txt");

fn part_1(input: &str) -> usize {
    let mut map = map::Map::from(input);
    map.get_guard_path_count()
}

fn part_2(input: &str) -> usize {
    let mut map = map::Map::from(input);
    let mut num_loops = 0;

    for row in 0..map.grid.len() {
        for col in 0..map.grid[row].len() {
            if (row, col) == map.guard_coord || map.grid[row][col] == map::Tile::Obstacle {
                continue;
            }

            map.grid[row][col] = map::Tile::Obstacle;
            if map.is_guard_path_loop() {
                num_loops += 1;
            }
            map.grid[row][col] = map::Tile::Empty;
            map.reset();
        }
    }

    num_loops
}

fn main() {
    println!("Part 1: {:?}", part_1(INPUT));
    println!("Part 2: {:?}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn test_example_part_1() {
        let result = part_1(EXAMPLE);
        assert_eq!(result, 41);
    }

    #[test]
    fn test_example_part_2() {
        let result = part_2(EXAMPLE);
        assert_eq!(result, 6);
    }
}
