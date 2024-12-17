use crate::coord::Coord;

#[derive(Debug)]
pub struct Robot {
    pub position: Coord,
    velocity: Coord,
}

impl Robot {
    pub fn pass_time(&mut self, seconds: usize, max_x: usize, max_y: usize) {
        let mut x = (self.position.x + self.velocity.x * seconds as isize) % max_x as isize;
        let mut y = (self.position.y + self.velocity.y * seconds as isize) % max_y as isize;

        if x < 0 {
            x += max_x as isize;
        }

        if y < 0 {
            y += max_y as isize;
        }

        self.position = Coord { x, y }
    }
}

impl From<&str> for Robot {
    fn from(s: &str) -> Self {
        let (p, v) = s.split_once(' ').unwrap();
        Robot {
            position: p[2..].into(),
            velocity: v[2..].into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut robot: Robot = "p=2,4 v=2,-3".into();

        robot.pass_time(1, 11, 7);
        assert_eq!(robot.position.to_tuple(), (4, 1));

        robot.pass_time(1, 11, 7);
        assert_eq!(robot.position.to_tuple(), (6, 5));

        robot.pass_time(1, 11, 7);
        assert_eq!(robot.position.to_tuple(), (8, 2));

        robot.pass_time(1, 11, 7);
        assert_eq!(robot.position.to_tuple(), (10, 6));

        robot.pass_time(1, 11, 7);
        assert_eq!(robot.position.to_tuple(), (1, 3));
    }
}
