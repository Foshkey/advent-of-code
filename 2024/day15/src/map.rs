use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Map {
    robot_position: (usize, usize),
    grid: Vec<Vec<Tile>>,
}

impl Map {
    pub fn move_robot(&mut self, d_row: isize, d_col: isize) {
        let mut valid_move = false;
        let mut has_box = false;
        let (mut row, mut col) = self.robot_position;

        loop {
            let Some(new_row) = row.checked_add_signed(d_row) else {
                break;
            };
            let Some(new_col) = col.checked_add_signed(d_col) else {
                break;
            };

            let next_tile = self.grid[new_row][new_col];
            if next_tile == Tile::Obstacle {
                break;
            }

            row = new_row;
            col = new_col;

            if next_tile == Tile::Empty {
                valid_move = true;
                break;
            }

            if next_tile == Tile::Box {
                has_box = true;
            }
        }

        if valid_move {
            let (robot_row, robot_col) = self.robot_position;
            let new_row = (robot_row as isize + d_row) as usize;
            let new_col = (robot_col as isize + d_col) as usize;
            self.robot_position = (new_row, new_col);

            if has_box {
                self.grid[row][col] = Tile::Box;
                self.grid[new_row][new_col] = Tile::Empty;
            }
        }
    }

    pub fn get_gps_sum(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .map(|(row, line)| {
                line.iter()
                    .enumerate()
                    .map(|(col, &tile)| {
                        if tile == Tile::Box {
                            row * 100 + col
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        let mut robot_position = (0, 0);
        let grid = s
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.char_indices()
                    .map(|(col, c)| {
                        if c == '@' {
                            robot_position = (row, col);
                        }

                        c.into()
                    })
                    .collect()
            })
            .collect();

        Map {
            robot_position,
            grid,
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (robot_row, robot_col) = self.robot_position;

        for (row, line) in self.grid.iter().enumerate() {
            for (col, &tile) in line.iter().enumerate() {
                if row == robot_row && col == robot_col {
                    write!(f, "@")?;
                    continue;
                }

                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Obstacle,
    Box,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::Obstacle,
            'O' => Tile::Box,
            _ => Tile::Empty,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Box => write!(f, "O"),
            Tile::Obstacle => write!(f, "#"),
            Tile::Empty => write!(f, "."),
        }
    }
}
