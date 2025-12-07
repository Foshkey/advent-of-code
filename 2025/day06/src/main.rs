use crate::homework::Homework;

mod homework;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let homework: Homework = input.into();
    homework.solve()
}

fn part_2(input: &str) -> usize {
    let homework = Homework::parse_cephalopod(input);
    homework.solve()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE), 4277556);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 3263827);
    }
}
