#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Keypad {
    Numeric,
    Directional,
}

impl Keypad {
    pub fn get_coord(&self, c: char) -> Coord {
        match (self, c) {
            (Keypad::Numeric, '7') => Coord { row: 0, col: 0 },
            (Keypad::Numeric, '8') => Coord { row: 0, col: 1 },
            (Keypad::Numeric, '9') => Coord { row: 0, col: 2 },
            (Keypad::Numeric, '4') => Coord { row: 1, col: 0 },
            (Keypad::Numeric, '5') => Coord { row: 1, col: 1 },
            (Keypad::Numeric, '6') => Coord { row: 1, col: 2 },
            (Keypad::Numeric, '1') => Coord { row: 2, col: 0 },
            (Keypad::Numeric, '2') => Coord { row: 2, col: 1 },
            (Keypad::Numeric, '3') => Coord { row: 2, col: 2 },
            (Keypad::Numeric, '0') => Coord { row: 3, col: 1 },
            (Keypad::Numeric, 'A') => Coord { row: 3, col: 2 },
            (Keypad::Directional, '^') => Coord { row: 0, col: 1 },
            (Keypad::Directional, 'A') => Coord { row: 0, col: 2 },
            (Keypad::Directional, '<') => Coord { row: 1, col: 0 },
            (Keypad::Directional, 'v') => Coord { row: 1, col: 1 },
            (Keypad::Directional, '>') => Coord { row: 1, col: 2 },
            _ => panic!("Unrecognized character {c}"),
        }
    }

    pub fn get_gap(&self) -> Coord {
        match self {
            Keypad::Numeric => Coord { row: 3, col: 0 },
            Keypad::Directional => Coord { row: 0, col: 0 },
        }
    }
}
