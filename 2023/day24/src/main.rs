use std::ops::Range;

use space::Space;

mod hailstone;
mod space;
mod vec3;

fn main() {
    let input = include_str!("../input.txt");
    println!(
        "Part 1: {}",
        part_1(input, 200000000000000..400000000000000)
    );
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str, test_area: Range<i128>) -> usize {
    let space = Space::from(input);
    space.find_2d_collisions(test_area)
}

fn part_2(input: &str) -> i128 {
    let space = Space::from(input);
    space.find_rock()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let example = "\
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        assert_eq!(part_2(example), 47);
    }
}
