#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    pub fn new(row: usize, col: usize) -> Self {
        Coord { row, col }
    }

    pub fn touches(&self, plot: Coord) -> bool {
        (self.row == plot.row && self.col.abs_diff(plot.col) == 1)
            || (self.col == plot.col && self.row.abs_diff(plot.row) == 1)
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_touches() {
        let coord1 = Coord { row: 2, col: 3 };
        let coord2 = Coord { row: 2, col: 4 };
        let coord3 = Coord { row: 3, col: 3 };
        let coord4 = Coord { row: 2, col: 2 };
        let coord5 = Coord { row: 1, col: 3 };
        let coord6 = Coord { row: 3, col: 4 };

        assert!(coord1.touches(coord2));
        assert!(coord1.touches(coord3));
        assert!(coord1.touches(coord4));
        assert!(coord1.touches(coord5));
        assert!(!coord1.touches(coord6));
    }
}
