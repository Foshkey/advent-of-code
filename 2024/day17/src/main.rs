use computer::Computer;

mod computer;

const INPUT: &str = include_str!("../input.txt");

fn part_1(input: &str) -> String {
    let mut program: Computer = input.into();
    program.execute();
    program
        .output
        .iter()
        .map(|&b| b.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn part_2(input: &str) -> usize {
    let program: Computer = input.into();
    program.find_a()
}

fn main() {
    println!("Part 1: {:?}", part_1(INPUT));
    println!("Part 2: {:?}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let example = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!(&part_1(example), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_example_2() {
        let example = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        assert_eq!(part_2(example), 117440);
    }
}
