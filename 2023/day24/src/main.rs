use std::ops::Range;

use hailstone::Hailstone;

mod hailstone;

fn main() {
    let input = include_str!("../input.txt");
    println!(
        "Part 1: {}",
        part_1(input, 200000000000000..400000000000000)
    );
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str, test_area: Range<i64>) -> usize {
    let mut count = 0;
    let hailstones: Vec<Hailstone> = input.lines().map(|line| line.into()).collect();
    for (i1, h1) in hailstones.iter().enumerate() {
        for (i2, h2) in hailstones.iter().enumerate() {
            if i1 >= i2 {
                continue;
            }

            let Some((x, y)) = h1.find_2d_collision(h2) else {
                continue;
            };

            if test_area.contains(&x) && test_area.contains(&y) {
                count += 1;
            }
        }
    }
    count
}

fn part_2(input: &str) -> usize {
    input.len()
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
        assert_eq!(part_1(example, 7..27), 2);
        //assert_eq!(part_2(example), 0);
    }
}
