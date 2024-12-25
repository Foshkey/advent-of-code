use crate::types::{Coord, Keypad};

pub struct Robot {
    keypad: Keypad,
    position: Coord,
    gap: Coord,
}

impl Robot {
    pub fn new(keypad: Keypad) -> Self {
        Robot {
            keypad,
            position: keypad.get_coord('A'),
            gap: keypad.get_gap(),
        }
    }

    pub fn enter_code(&mut self, code: &str) -> String {
        code.chars().map(|c| self.press(c)).collect::<String>()
    }

    fn press(&mut self, digit: char) -> String {
        let target = self.keypad.get_coord(digit);
        let path = self.get_path(target);
        self.position = target;
        path + "A"
    }

    fn get_path(&self, target: Coord) -> String {
        let mut paths = Vec::new();
        let mut vertical = String::new();
        let mut horizontal = String::new();
        let Coord { mut row, mut col } = self.position;

        while row != target.row {
            if row < target.row {
                vertical.push('v');
                row += 1;
            } else {
                vertical.push('^');
                row -= 1;
            }
        }

        while col != target.col {
            if col < target.col {
                horizontal.push('>');
                col += 1;
            } else {
                horizontal.push('<');
                col -= 1;
            }
        }

        paths.push(vertical.clone() + &horizontal);
        paths.push(horizontal + &vertical);
        paths.sort_by(|a, b| {
            let a_priority = a
                .chars()
                .map(|c| match c {
                    '<' => 0,
                    'v' => 1,
                    '>' => 2,
                    '^' => 3,
                    _ => 4,
                })
                .collect::<Vec<_>>();
            let b_priority = b
                .chars()
                .map(|c| match c {
                    '<' => 0,
                    'v' => 1,
                    '>' => 2,
                    '^' => 3,
                    _ => 4,
                })
                .collect::<Vec<_>>();
            a_priority.cmp(&b_priority)
        });

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
    fn test_numeric() {
        let mut robot = Robot::new(Keypad::Numeric);
        let result = robot.enter_code("029A");
        assert_eq!(result, "<A^A>^^AvvvA");
    }

    #[test]
    fn test_directional() {
        let mut robot = Robot::new(Keypad::Directional);
        let result = robot.enter_code("<A^A>^^AvvvA");
        assert_eq!(result, "v<<A>>^A<A>AvA<^AA>A<vAAA>^A");
    }

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
