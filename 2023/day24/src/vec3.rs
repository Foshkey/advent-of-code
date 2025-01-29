#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: i128,
    pub y: i128,
    pub z: i128,
}

impl Vec3 {
    pub fn new(x: i128, y: i128, z: i128) -> Self {
        Vec3 { x, y, z }
    }

    pub fn add(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn sub(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn gcd(&self) -> Vec3 {
        fn gcd(a: i128, b: i128) -> i128 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }

        let gcd = gcd(gcd(self.x, self.y), self.z);
        let x = self.x / gcd;
        let y = self.y / gcd;
        let z = self.z / gcd;
        Vec3 { x, y, z }
    }

    pub fn sum(&self) -> i128 {
        self.x + self.y + self.z
    }
}
