use pathfinding::prelude::astar;

use crate::coord::Coord;
use crate::map::Map;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    direction: Coord,
    location: Coord,
    straight_line_steps: u32,
}

pub fn find_minimal_heatloss(map: &Map, min_steps: u32, max_steps: u32) -> u32 {
    let start = Position {
        direction: Coord { x: 1, y: 0 },
        location: Coord { x: 0, y: 0 },
        straight_line_steps: 0,
    };
    let size = map.get_size();
    let end = Coord {
        x: size.x - 1,
        y: size.y - 1,
    };

    let result = astar(
        &start,
        |p| get_valid_directions(map, p, min_steps, max_steps),
        |p| map.distance(&p.location, &end),
        |p| p.straight_line_steps >= min_steps && p.location == end,
    )
    .unwrap();

    display_path(map, &result.0);
    result.1
}

fn display_path(map: &Map, path: &Vec<Position>) {
    for (y, row) in map.data.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            let coord = Coord {
                x: x as i32,
                y: y as i32,
            };
            let mut found = false;
            for position in path {
                if position.location == coord {
                    let c = match position.direction {
                        Coord { x: 1, y: 0 } => '>',
                        Coord { x: -1, y: 0 } => '<',
                        Coord { x: 0, y: 1 } => 'v',
                        Coord { x: 0, y: -1 } => '^',
                        _ => '?',
                    };
                    print!("{}", c);
                    found = true;
                    break;
                }
            }
            if !found {
                print!("{}", value);
            }
        }
        println!();
    }
}

// Private

fn get_valid_directions(
    map: &Map,
    position: &Position,
    min_steps: u32,
    max_steps: u32,
) -> Vec<(Position, u32)> {
    let mut valid_directions = Vec::new();

    let Position {
        direction,
        location,
        straight_line_steps,
    } = position;

    // Check if moving straight is valid
    let straight_location = Coord {
        x: location.x + direction.x,
        y: location.y + direction.y,
    };
    if straight_line_steps < &max_steps && map.is_within(&straight_location) {
        let heatloss = map.get_value(&straight_location).unwrap() as u32;
        valid_directions.push((
            Position {
                direction: direction.clone(),
                location: straight_location,
                straight_line_steps: straight_line_steps + 1,
            },
            heatloss,
        ));
    }

    // Check if moving left is valid
    let left_direction = turn_left(direction);
    let left_location = Coord {
        x: location.x + left_direction.x,
        y: location.y + left_direction.y,
    };
    if (location == &Coord { x: 0, y: 0 } || straight_line_steps >= &min_steps)
        && map.is_within(&left_location)
    {
        let heatloss = map.get_value(&left_location).unwrap() as u32;
        valid_directions.push((
            Position {
                direction: left_direction,
                location: left_location,
                straight_line_steps: 1,
            },
            heatloss,
        ));
    }

    // Check if moving right is valid
    let right_direction = turn_right(direction);
    let right_location = Coord {
        x: location.x + right_direction.x,
        y: location.y + right_direction.y,
    };
    if (location == &Coord { x: 0, y: 0 } || straight_line_steps >= &min_steps)
        && map.is_within(&right_location)
    {
        let heatloss = map.get_value(&right_location).unwrap() as u32;
        valid_directions.push((
            Position {
                direction: right_direction,
                location: right_location,
                straight_line_steps: 1,
            },
            heatloss,
        ));
    }

    valid_directions
}

fn turn_left(direction: &Coord) -> Coord {
    Coord {
        x: -direction.y,
        y: direction.x,
    }
}

fn turn_right(direction: &Coord) -> Coord {
    Coord {
        x: direction.y,
        y: -direction.x,
    }
}
