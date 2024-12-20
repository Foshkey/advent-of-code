use memory::{Coord, Memory};

mod memory;

const INPUT: &str = include_str!("../input.txt");

fn part_1(memory_size: usize, corruption_size: usize, input: &str) -> usize {
    let mut memory = Memory::new(memory_size, input);
    memory.corrupt(corruption_size);
    memory.find_path().unwrap()
}

fn part_2(memory_size: usize, corruption_size: usize, input: &str) -> Coord {
    let mut memory = Memory::new(memory_size, input);
    memory.corrupt(corruption_size + 1);
    while memory.find_path().is_some() {
        memory.corrupt_next();
    }
    memory.get_last_corruption().unwrap()
}

fn main() {
    println!("Part 1: {}", part_1(71, 1024, INPUT));
    println!("Part 2: {}", part_2(71, 1024, INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_example_1() {
        assert_eq!(part_1(7, 12, EXAMPLE), 22);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(part_2(7, 12, EXAMPLE).to_string(), "6,1");
    }
}
