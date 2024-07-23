use std::collections::BTreeSet;

use crate::{instruction::Instruction, point::Point, rectangle::Rectangle};

pub struct Map {
    points: BTreeSet<Point>,
}

impl Map {
    pub fn new<I>(instructions: I) -> Self
    where
        I: IntoIterator<Item = Instruction>,
    {
        let mut points = BTreeSet::new();

        let mut x = 0i64;
        let mut y = 0i64;

        for instruction in instructions {
            let Instruction {
                direction,
                length,
                color: _color,
            } = instruction;

            // Move x, y with the given direction and length
            x += direction.x as i64 * length as i64;
            y += direction.y as i64 * length as i64;

            // Create new point with direction
            points.insert(Point::new(x, y));
        }

        Map { points }
    }

    pub fn count_filled(&self) -> u64 {
        let mut count = 0u64;
        let mut curr_y = None;
        let mut lines = BTreeSet::new();
        let mut prev_rectangles = Vec::new();

        for point in self.points.iter() {
            let Point { x, y } = point;

            // Check if this is a new line
            if Some(y) != curr_y {
                // Build out some rectangles back to the previous y
                if let Some(prev_y) = curr_y {
                    let mut rectangles = Vec::new();
                    let mut paired_line = None;
                    for line in lines.iter() {
                        if let Some(prev_line) = paired_line {
                            // Create rectangle
                            let rect = Rectangle::new(
                                prev_line,
                                *prev_y,
                                (line - prev_line + 1) as u64,
                                (y - prev_y + 1) as u64,
                            );

                            // Add the area to the count
                            count += rect.area();

                            // Minus any intersections with previous rectangles
                            for prev_rect in prev_rectangles.iter() {
                                count -= rect.intersection(prev_rect);
                            }

                            // Push the rectangle for future intersections
                            rectangles.push(rect);
                            paired_line = None;
                        } else {
                            paired_line = Some(*line);
                        }
                    }
                    prev_rectangles = rectangles;
                }

                curr_y = Some(y);
            }

            if lines.contains(x) {
                lines.remove(x);
            } else {
                lines.insert(*x);
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::direction::Direction;

    use super::*;

    #[test]
    fn test_map_new() {
        let instructions = vec![
            Instruction {
                direction: Direction::new('L').unwrap(),
                length: 6,
                color: 0,
            },
            Instruction {
                direction: Direction::new('D').unwrap(),
                length: 8,
                color: 0,
            },
            Instruction {
                direction: Direction::new('R').unwrap(),
                length: 6,
                color: 0,
            },
            Instruction {
                direction: Direction::new('U').unwrap(),
                length: 8,
                color: 0,
            },
        ];

        let map = Map::new(instructions);

        let expected = vec![
            Point { x: -6, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: -6, y: 8 },
            Point { x: 0, y: 8 },
        ];

        assert_eq!(expected, map.points.into_iter().collect::<Vec<Point>>());
    }
}
