pub struct Rectangle {
    x: i64,
    y: i64,
    width: u64,
    height: u64,
}

impl Rectangle {
    pub fn new(x: i64, y: i64, width: u64, height: u64) -> Self {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }

    pub fn area(&self) -> u64 {
        self.width * self.height
    }

    pub fn intersection(&self, other: &Rectangle) -> u64 {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let width = (self.x + self.width as i64).min(other.x + other.width as i64) - x;
        let height = (self.y + self.height as i64).min(other.y + other.height as i64) - y;

        if width > 0 && height > 0 {
            (width * height) as u64
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let rectangle = Rectangle::new(0, 0, 5, 10);
        assert_eq!(rectangle.area(), 50);
    }

    #[test]
    fn test_intersection() {
        let rect = Rectangle::new(0, 0, 5, 5);
        let other = Rectangle::new(3, 3, 5, 5);
        assert_eq!(rect.intersection(&other), 4);

        let rect = Rectangle::new(0, 0, 5, 5);
        let other = Rectangle::new(6, 6, 5, 5);
        assert_eq!(rect.intersection(&other), 0);

        let rect = Rectangle::new(0, 0, 5, 5);
        let other = Rectangle::new(2, 2, 3, 3);
        assert_eq!(rect.intersection(&other), 9);

        let rect = Rectangle::new(0, 0, 5, 5);
        let other = Rectangle::new(0, 0, 5, 5);
        assert_eq!(rect.intersection(&other), 25);

        let rect = Rectangle::new(-5, -2, 5, 5);
        let other = Rectangle::new(-3, 2, 5, 5);
        assert_eq!(rect.intersection(&other), 3);
    }
}
