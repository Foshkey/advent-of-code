pub struct Hailstone {
    position: (i64, i64, i64),
    velocity: (i64, i64, i64),
}

impl Hailstone {
    pub fn find_2d_collision(&self, other: &Hailstone) -> Option<(i64, i64)> {
        let (x1, y1, _) = self.position;
        let (dx1, dy1, _) = self.velocity;
        let (x2, y2, _) = other.position;
        let (dx2, dy2, _) = other.velocity;

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

        Some((intersection_x, intersection_y))
    }
}

impl From<&str> for Hailstone {
    fn from(s: &str) -> Self {
        let (p_str, v_str) = s.split_once('@').unwrap();
        let mut p = p_str.split(',');
        let mut v = v_str.split(',');

        Self {
            position: (
                p.next().unwrap().trim().parse().unwrap(),
                p.next().unwrap().trim().parse().unwrap(),
                p.next().unwrap().trim().parse().unwrap(),
            ),
            velocity: (
                v.next().unwrap().trim().parse().unwrap(),
                v.next().unwrap().trim().parse().unwrap(),
                v.next().unwrap().trim().parse().unwrap(),
            ),
        }
    }
}
