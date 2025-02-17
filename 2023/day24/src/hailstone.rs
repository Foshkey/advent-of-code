use crate::vec3::Vec3;

pub struct Hailstone {
    pub position: Vec3,
    pub velocity: Vec3,
}

impl Hailstone {
    pub fn find_2d_collision(&self, other: &Hailstone) -> Option<Vec3> {
        let Vec3 { x: x1, y: y1, z: _ } = self.position;
        let Vec3 {
            x: dx1,
            y: dy1,
            z: _,
        } = self.velocity;
        let Vec3 { x: x2, y: y2, z: _ } = other.position;
        let Vec3 {
            x: dx2,
            y: dy2,
            z: _,
        } = other.velocity;

        // Calculate denominator first to check for parallel lines
        let denominator = dy2 * dx1 - dy1 * dx2;
        if denominator == 0 {
            return None; // Lines are parallel
        }

        let t = ((x2 - x1) * dy2 - (y2 - y1) * dx2) / denominator;
        let intersection_x = x1 + dx1 * t;
        let intersection_y = y1 + dy1 * t;

        // Check if intersection happens in the future for both stones
        let t2 = (intersection_x - x2) / dx2;
        if t < 0 || t2 < 0 {
            return None; // Intersection in the past
        }

        Some(Vec3 {
            x: intersection_x,
            y: intersection_y,
            z: 0,
        })
    }
}

impl From<&str> for Hailstone {
    fn from(s: &str) -> Self {
        let (p_str, v_str) = s.split_once('@').unwrap();
        let mut p = p_str.split(',');
        let mut v = v_str.split(',');

        Self {
            position: Vec3::new(
                p.next().unwrap().trim().parse().unwrap(),
                p.next().unwrap().trim().parse().unwrap(),
                p.next().unwrap().trim().parse().unwrap(),
            ),
            velocity: Vec3::new(
                v.next().unwrap().trim().parse().unwrap(),
                v.next().unwrap().trim().parse().unwrap(),
                v.next().unwrap().trim().parse().unwrap(),
            ),
        }
    }
}
