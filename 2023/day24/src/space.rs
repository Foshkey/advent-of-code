use crate::hailstone::Hailstone;
use std::ops::Range;

pub struct Space {
    hailstones: Vec<Hailstone>,
}

impl Space {
    pub fn find_2d_collisions(&self, test_area: Range<i128>) -> usize {
        self.hailstones
            .iter()
            .enumerate()
            .flat_map(|(i1, h1)| {
                self.hailstones
                    .iter()
                    .enumerate()
                    .filter(move |(i2, _)| i1 <= *i2)
                    .filter_map(|(_, h2)| h1.find_2d_collision(h2))
                    .filter(|p| test_area.contains(&p.x) && test_area.contains(&p.y))
            })
            .count()
    }

    pub fn find_rock(&self) -> i128 {
        // Take 3 arbitrary hailstones.
        let Hailstone {
            position: p0,
            velocity: v0,
        } = self.hailstones[0];
        let Hailstone {
            position: p1,
            velocity: v1,
        } = self.hailstones[1];
        let Hailstone {
            position: p2,
            velocity: v2,
        } = self.hailstones[2];

        // Subtract the positions and velocities to make them relative.
        // The first hailstone is stationary at the origin.
        let p3 = p1.sub(&p0);
        let p4 = p2.sub(&p0);
        let v3 = v1.sub(&v0);
        let v4 = v2.sub(&v0);

        // Find the normal to the plane that the second and third hailstones velocity lies in.
        // This is the cross product of their respective position and velocity.
        // The cross product `s` of these two vectors is the same direction but not necessarily the
        // same magnitude of the desired velocity of the rock.
        // Only the direction is relevant (not the magnitude) so we can normalize the vector by the
        // GCD of its components in order to prevent numeric overflow.
        let q = v3.cross(&p3).gcd();
        let r = v4.cross(&p4).gcd();
        let s = q.cross(&r).gcd();

        // Find the times when the second and third hailstone intercept this vector.
        // If the times are different then we can extrapolate the original position of the rock.
        let t = (p3.y * s.x - p3.x * s.y) / (v3.x * s.y - v3.y * s.x);
        let u = (p4.y * s.x - p4.x * s.y) / (v4.x * s.y - v4.y * s.x);
        assert!(t != u);

        // Calculate the original position of the rock, remembering to add the first hailstone's
        // position to convert back to absolute coordinates.
        let a = p0.add(&p3).sum();
        let b = p0.add(&p4).sum();
        let c = v3.sub(&v4).sum();
        (u * a - t * b + u * t * c) / (u - t)
    }
}

impl From<&str> for Space {
    fn from(s: &str) -> Self {
        Self {
            hailstones: s.lines().map(|line| line.into()).collect(),
        }
    }
}
