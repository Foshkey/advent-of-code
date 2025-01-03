use processor::Processor;

mod processor;

const INPUT: &str = include_str!("../input.txt");

fn part_1(input: &str) -> usize {
    let processor: Processor = input.parse().unwrap();
    processor.get_output()
}

fn part_2(input: &str) -> String {
    let processor: Processor = input.parse().unwrap();
    let mut swaps: Vec<_> = processor.get_wrong_gates().into_iter().collect();
    swaps.sort();
    swaps.join(",")
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
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
        assert_eq!(part_1(example), 4);
    }

    #[test]
    fn test_example_2() {
        let example = include_str!("../example.txt");
        assert_eq!(part_1(example), 2024);
    }
}
