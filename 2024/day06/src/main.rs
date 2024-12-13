mod map;

const INPUT: &str = include_str!("input.txt");

fn part_1(input: &str) -> usize {
    let map = map::Map::from(input);
    map.trace_guard_path().unwrap()
}

fn part_2(input: &str) -> usize {
    let map = map::Map::from(input);
    let mut num_loops = 0;

    for row in 0..map.grid.len() {
        for col in 0..map.grid[row].len() {
            if (row, col) == map.guard_coord || map.grid[row][col] == map::Tile::Obstacle {
                continue;
            }

            let mut new_map = map.clone();
            new_map.grid[row][col] = map::Tile::Obstacle;
            if matches!(new_map.trace_guard_path(), Err(map::MapError::LoopingPath)) {
                num_loops += 1;
            }
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
