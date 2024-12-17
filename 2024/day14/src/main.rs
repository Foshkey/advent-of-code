use room::Room;

mod coord;
mod robot;
mod room;

const INPUT: &str = include_str!("../input.txt");

fn part_1(input: &str, width: usize, height: usize) -> usize {
    let mut room = Room::new(input, width, height);
    room.pass_time(100);
    room.get_safety_factor()
}

fn part_2(input: &str, width: usize, height: usize) -> usize {
    let mut room = Room::new(input, width, height);
    let mut count = 0;
    while !room.is_easter_egg() {
        room.pass_time(1);
        count += 1;
    }
    println!("{}", room);
    count
}

fn main() {
    println!("Part 1: {:?}", part_1(INPUT, 101, 103));
    println!("Part 2: {:?}", part_2(INPUT, 101, 103));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let example = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!(part_1(example, 11, 7), 12);
    }
}
