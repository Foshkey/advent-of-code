use std::fmt::{Display, Formatter, Result};

use crate::robot::Robot;

#[derive(Debug)]
pub struct Room {
    robots: Vec<Robot>,
    width: usize,
    height: usize,
}

impl Room {
    pub fn new(input: &str, width: usize, height: usize) -> Self {
        Room {
            robots: input.lines().map(|line| line.into()).collect(),
            width,
            height,
        }
    }

    pub fn pass_time(&mut self, seconds: usize) {
        for robot in self.robots.iter_mut() {
            robot.pass_time(seconds, self.width, self.height);
        }
    }

    pub fn get_safety_factor(&self) -> usize {
        let mid_x = self.width as isize / 2;
        let mid_y = self.height as isize / 2;
        let mut quadrants = [0, 0, 0, 0];

        for robot in &self.robots {
            match robot.position.to_tuple() {
                (x, y) if x < mid_x && y < mid_y => quadrants[0] += 1,
                (x, y) if x > mid_x && y < mid_y => quadrants[1] += 1,
                (x, y) if x < mid_x && y > mid_y => quadrants[2] += 1,
                (x, y) if x > mid_x && y > mid_y => quadrants[3] += 1,
                _ => (),
            }
        }

        quadrants.iter().product()
    }

    pub fn is_easter_egg(&self) -> bool {
        let mid_x_1 = self.width as isize / 3;
        let mid_x_2 = mid_x_1 * 2;
        let mid_y_1 = self.height as isize / 3;
        let mid_y_2 = mid_y_1 * 2;
        let robots_near_middle = self
            .robots
            .iter()
            .filter(|r| {
                r.position.x > mid_x_1
                    && r.position.x < mid_x_2
                    && r.position.y > mid_y_1
                    && r.position.y < mid_y_2
            })
            .count();
        robots_near_middle > 250
    }
}

impl Display for Room {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut grid = vec![vec![false; self.width]; self.height];

        for robot in &self.robots {
            grid[robot.position.y as usize][robot.position.x as usize] = true;
        }

        for row in grid {
            for is_robot in row {
                write!(f, "{}", if is_robot { 'X' } else { '.' })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
