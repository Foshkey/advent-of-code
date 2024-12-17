#[derive(Debug)]
pub struct ClawMachine {
    button_a: Coord,
    button_b: Coord,
    prize: Coord,
}

impl ClawMachine {
    pub fn get_minimum_tokens(&self) -> Option<usize> {
        let mut tokens: Vec<usize> = Vec::new();
        let mut a_presses = 0;

        loop {
            a_presses += 1;
            let Some(x) = self.prize.x.checked_sub(self.button_a.x * a_presses) else {
                break;
            };
            let Some(y) = self.prize.y.checked_sub(self.button_a.y * a_presses) else {
                break;
            };

            if x % self.button_b.x == 0 && y % self.button_b.y == 0 {
                let b_presses = x / self.button_b.x;
                if b_presses == y / self.button_b.y {
                    tokens.push(3 * a_presses + b_presses)
                }
            }
        }

        if tokens.is_empty() {
            return None;
        }

        Some(
            tokens
                .iter()
                .fold(usize::MAX, |min, &x| if x < min { x } else { min }),
        )
    }

    pub fn move_prize_position(&mut self, d_x: usize, d_y: usize) {
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
    x: usize,
    y: usize,
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
