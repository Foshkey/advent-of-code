use world::World;

mod brick;
mod world;

fn part_1(input: &str) -> usize {
    let world: World = input.into();
    world.count_safe()
}

fn part_2(input: &str) -> usize {
    let world: World = input.into();
    world.count_chain_reactions()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let example = "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        assert_eq!(part_1(example), 5);
        assert_eq!(part_2(example), 7);
    }

    #[test]
    fn test_example_2() {
        let example = "\
0,0,1~0,1,1
1,1,1~1,1,1
0,0,2~0,0,2
0,1,2~1,1,2";
        assert_eq!(part_1(example), 3);
    }

    #[test]
    fn test_example_3() {
        let example = "\
0,0,1~1,0,1
0,1,1~0,1,2
0,0,5~0,0,5
0,0,4~0,1,4";
        assert_eq!(part_1(example), 2);
    }
}
