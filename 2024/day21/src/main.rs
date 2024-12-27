use robot::Robot;
use types::Keypad;

mod robot;
mod types;

const INPUT: &str = include_str!("../input.txt");

fn part_1(input: &str) -> usize {
    let numeric_robot = Robot::new(Keypad::Numeric);
    let mut directional_robot = Robot::new(Keypad::Directional);

    input
        .lines()
        .map(|line| {
            let directions = numeric_robot.get_directions(line);
            let cost = directional_robot.get_cost(&directions, 2);
            let num = line[0..3].parse::<usize>().unwrap();
            cost * num
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let numeric_robot = Robot::new(Keypad::Numeric);
    let mut directional_robot = Robot::new(Keypad::Directional);

    input
        .lines()
        .map(|line| {
            let directions = numeric_robot.get_directions(line);
            let cost = directional_robot.get_cost(&directions, 25);
            let num = line[0..3].parse::<usize>().unwrap();
            cost * num
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
