use crate::database::Database;

mod database;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let database: Database = input.into();
    database.count_available_fresh()
}

fn part_2(input: &str) -> usize {
    let mut database: Database = input.into();
    database.count_all_fresh()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 14);
    }
}
