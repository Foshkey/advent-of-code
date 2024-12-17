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
}
