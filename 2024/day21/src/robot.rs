use std::collections::HashMap;

use crate::types::{Coord, Keypad};

pub struct Robot {
    keypad: Keypad,
    position: Coord,
    gap: Coord,
    cache: HashMap<(Coord, Coord), String>,
}

impl Robot {
    pub fn new(keypad: Keypad) -> Self {
        Robot {
            keypad,
            position: keypad.get_coord('A'),
            gap: keypad.get_gap(),
            cache: HashMap::new(),
        }
    }

    pub fn enter_code(&mut self, code: &str) -> String {
        code.chars().map(|c| self.press(c)).collect::<String>()
    }

    fn press(&mut self, digit: char) -> String {
        let target = self.keypad.get_coord(digit);

        if let Some(cached_result) = self.cache.get(&(self.position, target)) {
            self.position = target;
            return cached_result.clone();
        }

        let result = self.get_path(target) + "A";
        self.cache.insert((self.position, target), result.clone());
        self.position = target;
        result
    }

    fn get_path(&self, target: Coord) -> String {
        let mut paths = Vec::new();
        let Coord { row, col } = self.position;
        let mut horizontal_first = false;

        let vertical = if row < target.row {
            "v".repeat(target.row - row)
        } else {
            "^".repeat(row - target.row)
        };

        let horizontal = if col < target.col {
            ">".repeat(target.col - col)
        } else {
            horizontal_first = true;
            "<".repeat(col - target.col)
        };

        if horizontal_first {
            paths.push(horizontal.clone() + &vertical);
            paths.push(vertical + &horizontal);
        } else {
            paths.push(vertical.clone() + &horizontal);
            paths.push(horizontal + &vertical);
        }

        paths.into_iter().find(|p| self.is_path_clear(p)).unwrap()
    }

    fn is_path_clear(&self, directions: &str) -> bool {
        let mut new_position = self.position;
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
        let mut numeric_robot = Robot::new(Keypad::Numeric);
        let mut directional_robot = Robot::new(Keypad::Directional);

        let result = numeric_robot.enter_code("029A");
        let result = directional_robot.enter_code(&result);
        let result = directional_robot.enter_code(&result);
        assert_eq!(result.len(), 68);
        let result = numeric_robot.enter_code("980A");
        let result = directional_robot.enter_code(&result);
        let result = directional_robot.enter_code(&result);
        assert_eq!(result.len(), 60);
        let result = numeric_robot.enter_code("179A");
        let result = directional_robot.enter_code(&result);
        let result = directional_robot.enter_code(&result);
        assert_eq!(result.len(), 68);
        let result = numeric_robot.enter_code("456A");
        let result = directional_robot.enter_code(&result);
        let result = directional_robot.enter_code(&result);
        assert_eq!(result.len(), 64);
        let result = numeric_robot.enter_code("379A");
        let result = directional_robot.enter_code(&result);
        let result = directional_robot.enter_code(&result);
        assert_eq!(result.len(), 64);
    }
}
