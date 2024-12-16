mod coord;
mod map;
mod region;

const INPUT: &str = include_str!("../input.txt");

fn part_1(input: &str) -> usize {
    let map: map::Map = input.into();
    map.get_fence_cost()
}

fn part_2(input: &str) -> usize {
    input.len()
}

fn main() {
    println!("Part 1: {:?}", part_1(INPUT));
    println!("Part 2: {:?}", part_2(INPUT));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_small_example() {
        let example = "\
AAAA
BBCD
BBCC
EEEC";
        assert_eq!(part_1(example), 140);
        //assert_eq!(part_2(example), 1206);
    }

    #[test]
    fn test_xo_example() {
        let example = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!(part_1(example), 772);
        //assert_eq!(part_2(example), 1206);
    }

    #[test]
    fn test_example() {
        let example = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(part_1(example), 1930);
        //assert_eq!(part_2(example), 1206);
    }
}
