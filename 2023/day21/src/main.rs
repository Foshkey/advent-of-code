use map::Map;

mod map;

fn main() {
    let map: Map = include_str!("../input.txt").into();
    println!("Part 1: {}", map.get_num_spaces_1(64));
    println!("Part 2: {}", map.get_num_spaces_2(26501365));
}
