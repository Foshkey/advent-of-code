const INPUT: &str = include_str!("input.txt");

struct Cell {
    value: char,
    row: usize,
    col: usize,
}

struct Puzzle<'a> {
    width: usize,
    height: usize,
    lines: Vec<&'a str>,
}

impl<'a> From<&'a str> for Puzzle<'a> {
    fn from(value: &'a str) -> Self {
        let lines: Vec<&str> = value.lines().collect();

        Puzzle {
            width: lines[0].len(),
            height: lines.len(),
            lines,
        }
    }
}

impl<'a> Puzzle<'a> {
    fn get_next(&self, cell: &Cell, d_row: i32, d_col: i32) -> Option<Cell> {
        let row = cell.row as i32 + d_row;
        if row < 0 || row as usize >= self.height {
            return None;
        }
        let row = row as usize;

        let col = cell.col as i32 + d_col;
        if col < 0 || col as usize >= self.width {
            return None;
        }
        let col = col as usize;

        Some(Cell {
            row,
            col,
            value: self.lines[row].chars().nth(col).unwrap(),
        })
    }

    fn is_xmas(&self, cell: &Cell, d_row: i32, d_col: i32) -> bool {
        if cell.value == 'S' {
            return true;
        }

        let Some(next_value) = "XMAS"
            .find(cell.value)
            .and_then(|i| "XMAS".chars().nth(i + 1))
        else {
            return false;
        };

        let Some(next_cell) = self.get_next(cell, d_row, d_col) else {
            return false;
        };
        if next_cell.value != next_value {
            return false;
        }

        self.is_xmas(&next_cell, d_row, d_col)
    }

    fn is_x_of_mas(&self, cell: &Cell) -> bool {
        let Some(top_left) = self.get_next(cell, -1, -1) else {
            return false;
        };
        let Some(top_right) = self.get_next(cell, -1, 1) else {
            return false;
        };
        let Some(bottom_left) = self.get_next(cell, 1, -1) else {
            return false;
        };
        let Some(bottom_right) = self.get_next(cell, 1, 1) else {
            return false;
        };

        match (top_left.value, bottom_right.value) {
            ('M', 'S') => (),
            ('S', 'M') => (),
            _ => return false,
        };

        match (top_right.value, bottom_left.value) {
            ('M', 'S') => (),
            ('S', 'M') => (),
            _ => return false,
        };

        true
    }
}

fn part_1(input: &str) -> usize {
    let puzzle = Puzzle::from(input);
    let mut sum = 0;

    for (row, &line) in puzzle.lines.iter().enumerate() {
        for (col, c) in line.char_indices() {
            if c != 'X' {
                continue;
            }

            let cell = Cell { row, col, value: c };

            for d_row in -1..=1 {
                for d_col in -1..=1 {
                    if puzzle.is_xmas(&cell, d_row, d_col) {
                        sum += 1
                    }
                }
            }
        }
    }

    sum
}

fn part_2(input: &str) -> usize {
    let puzzle = Puzzle::from(input);
    let mut sum = 0;

    for (row, &line) in puzzle.lines.iter().enumerate() {
        for (col, c) in line.char_indices() {
            if c != 'A' {
                continue;
            }

            let cell = Cell { row, col, value: c };

            if puzzle.is_x_of_mas(&cell) {
                sum += 1
            }
        }
    }

    sum
}

fn main() {
    println!("Part 1: {:?}", part_1(INPUT));
    println!("Part 2: {:?}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn test_example_part_1() {
        let result = part_1(EXAMPLE);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_example_part_2() {
        let result = part_2(EXAMPLE);
        assert_eq!(result, 9);
    }
}
