use std::{collections::HashMap, time::Instant};

type Instructions = Vec<Direction>;
type NextNode<'a> = (&'a str, &'a str);
type Map<'a> = HashMap<&'a str, NextNode<'a>>;

enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid value '{value}' for Direction, expected L or R"),
        }
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn load_instructions(line: &str) -> Vec<Direction> {
    line.chars().map(Direction::from).collect()
}

fn load_node<'a>(line: &'a str, nodes: &mut Map<'a>) {
    let id = &line[..3];
    let next_l = &line[7..10];
    let next_r = &line[12..15];
    nodes.insert(id, (next_l, next_r));
}

fn load_input(input: &str) -> (Instructions, Map) {
    let mut lines = input.lines();
    let instructions = load_instructions(lines.next().unwrap());
    let _ = lines.next(); // throw away empty line

    let mut nodes: Map = HashMap::new();
    for line in lines {
        load_node(line, &mut nodes);
    }

    (instructions, nodes)
}

fn step_to_end<F>(start: &str, instructions: &[Direction], map: &Map, is_end: F) -> u64
where
    F: Fn(&str) -> bool,
{
    let mut steps = 0u64;
    let mut current_node = map.get_key_value(start).unwrap();

    for inst in instructions.iter().cycle() {
        steps += 1;
        current_node = match inst {
            Direction::Left => map.get_key_value(current_node.1 .0).unwrap(),
            Direction::Right => map.get_key_value(current_node.1 .1).unwrap(),
        };

        if is_end(current_node.0) {
            break;
        }
    }

    steps
}

fn is_end(name: &str) -> bool {
    name == "ZZZ"
}

fn is_end_ghost(name: &str) -> bool {
    name.ends_with('Z')
}

fn get_total_steps(input: &str) -> u64 {
    let (instructions, map) = load_input(input);
    step_to_end("AAA", &instructions, &map, is_end)
}

fn get_simultaneous_steps(input: &str) -> u64 {
    let (instructions, map) = load_input(input);
    map.keys()
        .filter(|&id| id.ends_with('A'))
        .map(|&id| step_to_end(id, &instructions, &map, is_end_ghost))
        .fold(1u64, lcm)
}

fn main() {
    let now = Instant::now();

    let input = include_str!("input.txt");
    println!("Part 1: {}", get_total_steps(input));
    println!("Part 2: {}", get_simultaneous_steps(input));

    println!("Executed in {} μs", now.elapsed().as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = include_str!("example1.txt");
        assert_eq!(2, get_total_steps(input));
    }

    #[test]
    fn test_example2() {
        let input = include_str!("example2.txt");
        assert_eq!(6, get_total_steps(input));
    }

    #[test]
    fn test_example3() {
        let input = include_str!("example3.txt");
        assert_eq!(6, get_simultaneous_steps(input));
    }

    #[test]
    fn test_load_node() {
        let mut nodes: Map = HashMap::new();
        let input = "AAA = (BBB, CCC)";

        load_node(input, &mut nodes);

        assert_eq!(&("BBB", "CCC"), nodes.get("AAA").unwrap());
    }
}
