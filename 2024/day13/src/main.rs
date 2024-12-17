mod claw_machine;

const INPUT: &str = include_str!("../input.txt");

fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(claw_machine::ClawMachine::from)
        .filter_map(|machine| machine.get_minimum_tokens())
        .sum()
}

fn part_2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|s| {
            let mut machine = claw_machine::ClawMachine::from(s);
            machine.move_prize_position(10000000000000, 10000000000000);
            machine
        })
        .filter_map(|machine| machine.get_minimum_tokens())
        .sum()
}

fn main() {
    println!("Part 1: {:?}", part_1(INPUT));
    println!("Part 2: {:?}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let example = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!(part_1(example), 480);
    }
}
