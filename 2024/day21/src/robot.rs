use std::collections::HashMap;

use crate::types::{Coord, Keypad};

pub struct Robot {
    keypad: Keypad,
    gap: Coord,
    cache: HashMap<(String, usize), usize>,
}

impl Robot {
    pub fn new(keypad: Keypad) -> Self {
        Robot {
            keypad,
            gap: keypad.get_gap(),
            cache: HashMap::new(),
        }
    }

    pub fn get_directions(&self, code: &str) -> String {
        let mut position = self.keypad.get_coord('A');
        code.chars()
            .map(|c| {
                let target = self.keypad.get_coord(c);
                let result = self.get_path(position, target) + "A";
                position = target;
                result
            })
            .collect::<String>()
    }

    pub fn get_cost(&mut self, code: &str, robots: usize) -> usize {
        // If there are 0 robots left, the cost is the length of the code
        if robots == 0 {
            return code.len();
        }

        // Check if it's in the cache
        if let Some(&cached_cost) = self.cache.get(&(code.to_string(), robots)) {
            return cached_cost;
        }

        // Start the arm position at A and start counting
        let mut position = self.keypad.get_coord('A');
        let mut count = 0;
        for c in code.chars() {
            // Figure out the target coordinate
            let target = self.keypad.get_coord(c);
            // Get the path, followed by A
            let result = self.get_path(position, target) + "A";
            // Calculate the cost of the result (recursively)
            count += self.get_cost(&result, robots - 1);
            // Finally make sure that the new position is set for the next iteration
            position = target;
        }

        // Insert into the cache
        self.cache.insert((code.to_string(), robots), count);

        // Return
        count
    }

    fn get_path(&self, position: Coord, target: Coord) -> String {
        // Observations about the optimal path:
        // 1. Move as far as possible in one direction, then the next
        // 2. Get to < first if possible, since it's the furthest from A
        // 3. Check if we move over the gap
        let mut paths = Vec::new();
        let Coord { row, col } = position;
        let mut horizontal_first = false;

        // Move up or down
        let vertical = if row < target.row {
            "v".repeat(target.row - row)
        } else {
            "^".repeat(row - target.row)
        };

        // Move left or right
        let horizontal = if col < target.col {
            ">".repeat(target.col - col)
        } else {
            // If this is the case, then we want to move horizontally first
            horizontal_first = true;
            "<".repeat(col - target.col)
        };

        // Build up the possible paths
        if horizontal_first {
            paths.push(horizontal.clone() + &vertical);
            paths.push(vertical + &horizontal);
        } else {
            paths.push(vertical.clone() + &horizontal);
            paths.push(horizontal + &vertical);
        }

        // Find one that is clear (doesn't go over the gap)
        paths
            .into_iter()
            .find(|p| self.is_path_clear(position, p))
            .unwrap()
    }

    fn is_path_clear(&self, position: Coord, directions: &str) -> bool {
        // This just goes through the motions of the directions and checks if
        // the arm every goes over the gap. If that's the case, then invalid
        let mut new_position = position;
        for direction in directions.chars() {
            let next = match direction {
                '^' => Coord {
                    row: new_position.row - 1,
                    ..new_position
                },
                'v' => Coord {
                    row: new_position.row + 1,
                    ..new_position
                },
                '<' => Coord {
                    col: new_position.col - 1,
                    ..new_position
                },
                '>' => Coord {
                    col: new_position.col + 1,
                    ..new_position
                },
                _ => return false,
            };
            if next == self.gap {
                return false;
            }
            new_position = next;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combo() {
        let numeric_robot = Robot::new(Keypad::Numeric);
        let mut directional_robot = Robot::new(Keypad::Directional);

        let result = numeric_robot.get_directions("029A");
        let result = directional_robot.get_cost(&result, 2);
        assert_eq!(result, 68);
        let result = numeric_robot.get_directions("980A");
        let result = directional_robot.get_cost(&result, 2);
        assert_eq!(result, 60);
        let result = numeric_robot.get_directions("179A");
        let result = directional_robot.get_cost(&result, 2);
        assert_eq!(result, 68);
        let result = numeric_robot.get_directions("456A");
        let result = directional_robot.get_cost(&result, 2);
        assert_eq!(result, 64);
        let result = numeric_robot.get_directions("379A");
        let result = directional_robot.get_cost(&result, 2);
        assert_eq!(result, 64);
    }
}
