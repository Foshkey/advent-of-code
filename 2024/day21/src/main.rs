use robot::Robot;
use types::Keypad;

mod robot;
mod types;

const INPUT: &str = include_str!("../input.txt");

fn part_1(input: &str) -> usize {
    let mut numeric_robot = Robot::new(Keypad::Numeric);
    let mut directional_robot = Robot::new(Keypad::Directional);

    input
        .lines()
        .map(|line| {
            let result = numeric_robot.enter_code(line);
            let result = directional_robot.enter_code(&result);
            let result = directional_robot.enter_code(&result);
            let num = line[0..3].parse::<usize>().unwrap();
            result.len() * num
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let mut numeric_robot = Robot::new(Keypad::Numeric);
    let mut directional_robot = Robot::new(Keypad::Directional);

    input
        .lines()
        .map(|line| {
            let mut result = numeric_robot.enter_code(line);
            for _ in 0..25 {
                result = directional_robot.enter_code(&result);
            }
            let num = line[0..3].parse::<usize>().unwrap();
            result.len() * num
        })
        .sum()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let example = "\
029A
980A
179A
456A
379A";
        assert_eq!(part_1(example), 126384);
    }
}
