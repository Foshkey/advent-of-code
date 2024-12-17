#[derive(Debug)]
pub struct ClawMachine {
    button_a: Coord,
    button_b: Coord,
    prize: Coord,
}

impl ClawMachine {
    pub fn get_minimum_tokens(&self) -> Option<usize> {
        let main_determinant =
            self.button_a.x * self.button_b.y - self.button_a.y * self.button_b.x;
        let determinant_a = self.prize.x * self.button_b.y - self.prize.y * self.button_b.x;
        let determinant_b = self.prize.x * self.button_a.y - self.prize.y * self.button_a.x;

        if determinant_a % main_determinant != 0 || determinant_b % main_determinant != 0 {
            return None;
        }

        let a_presses = (determinant_a / main_determinant).unsigned_abs();
        let b_presses = (determinant_b / main_determinant).unsigned_abs();

        Some(a_presses * 3 + b_presses)
    }

    pub fn move_prize_position(&mut self, d_x: isize, d_y: isize) {
        self.prize.x += d_x;
        self.prize.y += d_y;
    }
}

impl From<&str> for ClawMachine {
    fn from(s: &str) -> Self {
        let mut lines = s.lines();

        // This is hilariously brittle but who cares it's AoC
        ClawMachine {
            button_a: lines.next().unwrap().into(),
            button_b: lines.next().unwrap().into(),
            prize: lines.next().unwrap().into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coord {
    x: isize,
    y: isize,
}

impl From<&str> for Coord {
    fn from(s: &str) -> Self {
        let index = s.find('X').unwrap();
        let (x_str, y_str) = s[index..].split_once(',').unwrap();

        Coord {
            x: x_str.trim()[2..].parse().unwrap(),
            y: y_str.trim()[2..].parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let machine: ClawMachine = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400
"
        .into();
        assert_eq!(machine.button_a, Coord { x: 94, y: 34 });
        assert_eq!(machine.button_b, Coord { x: 22, y: 67 });
        assert_eq!(machine.prize, Coord { x: 8400, y: 5400 });
    }
}
