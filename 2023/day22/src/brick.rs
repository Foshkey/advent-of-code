use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Brick {
    p1: Coord,
    p2: Coord,
}

impl Brick {
    pub fn max_z(&self) -> usize {
        self.p1.z.max(self.p2.z)
    }

    pub fn min_z(&self) -> usize {
        self.p1.z.min(self.p2.z)
    }

    pub fn will_collide(&self, other: &Self) -> bool {
        let self_min_x = self.p1.x.min(self.p2.x);
        let self_max_x = self.p1.x.max(self.p2.x);
        let self_min_y = self.p1.y.min(self.p2.y);
        let self_max_y = self.p1.y.max(self.p2.y);

        let other_min_x = other.p1.x.min(other.p2.x);
        let other_max_x = other.p1.x.max(other.p2.x);
        let other_min_y = other.p1.y.min(other.p2.y);
        let other_max_y = other.p1.y.max(other.p2.y);

        self_min_x <= other_max_x
            && self_max_x >= other_min_x
            && self_min_y <= other_max_y
            && self_max_y >= other_min_y
    }

    pub fn supports(&self, other: &Brick) -> bool {
        self.max_z() + 1 == other.min_z() && self.will_collide(other)
    }

    pub fn move_to(&mut self, z: usize) {
        let diff = self.p1.z.abs_diff(self.p2.z);
        if self.p1.z < self.p2.z {
            self.p1.z = z;
            self.p2.z = z + diff;
        } else {
            self.p2.z = z;
            self.p1.z = z + diff;
        }
    }
}

impl From<&str> for Brick {
    fn from(s: &str) -> Self {
        let (p1, p2) = s.split_once('~').unwrap();
        Brick {
            p1: p1.into(),
            p2: p2.into(),
        }
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        self.min_z()
            .cmp(&other.min_z())
            .then(self.max_z().cmp(&other.max_z()))
            .then(self.p1.cmp(&other.p1))
            .then(self.p2.cmp(&other.p2))
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

impl From<&str> for Coord {
    fn from(s: &str) -> Self {
        let n: Vec<_> = s.split(',').map(|s| s.parse().unwrap()).collect();
        Coord {
            x: n[0],
            y: n[1],
            z: n[2],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_will_collide() {
        let brick1 = Brick {
            p1: Coord { x: 0, y: 0, z: 0 },
            p2: Coord { x: 0, y: 0, z: 2 },
        };
        let brick2 = Brick {
            p1: Coord { x: 0, y: 1, z: 1 },
            p2: Coord { x: 0, y: 1, z: 3 },
        };
        let brick3 = Brick {
            p1: Coord { x: 0, y: 0, z: 5 },
            p2: Coord { x: 0, y: 5, z: 5 },
        };
        let brick4 = Brick {
            p1: Coord { x: 0, y: 0, z: 3 },
            p2: Coord { x: 2, y: 0, z: 3 },
        };

        assert!(!brick1.will_collide(&brick2));
        assert!(brick1.will_collide(&brick4));
        assert!(!brick2.will_collide(&brick4));
        assert!(brick2.will_collide(&brick3));
        assert!(brick3.will_collide(&brick4));
        assert!(brick3.will_collide(&brick1));
    }
}
