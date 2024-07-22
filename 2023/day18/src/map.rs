use std::fmt::Display;

use anyhow::Result;

use crate::instruction::Instructions;

pub struct Map {
    pub cells: Vec<Vec<u32>>,
}

impl Map {
    pub fn new(instructions: Instructions) -> Result<Self> {
        let mut cells = vec![vec![0; 1000]; 1000];
        let mut x = 500;
        let mut y = 500;

        for instruction in instructions {
            for _ in 0..instruction.length {
                x = (x as i32 + instruction.direction.x) as usize;
                y = (y as i32 + instruction.direction.y) as usize;
                cells[y][x] = instruction.color;
            }
        }

        let mut map = Map { cells };
        map.trim();

        Ok(map)
    }

    pub fn trim(&mut self) {
        let mut min_x = 1000;
        let mut min_y = 1000;
        let mut max_x = 0;
        let mut max_y = 0;

        for (y, row) in self.cells.iter().enumerate() {
            for (x, &color) in row.iter().enumerate() {
                if color != 0 {
                    min_x = min_x.min(x);
                    min_y = min_y.min(y);
                    max_x = max_x.max(x);
                    max_y = max_y.max(y);
                }
            }
        }

        let mut cells = vec![vec![0; max_x - min_x + 1]; max_y - min_y + 1];

        for (y, row) in self.cells[min_y..=max_y].iter().enumerate() {
            for (x, &color) in row[min_x..=max_x].iter().enumerate() {
                cells[y][x] = color;
            }
        }

        self.cells = cells
    }

    pub fn fill(&mut self) -> Result<()> {
        let mut updated_cells = self.cells.clone();

        for (y, row) in self.cells.iter().enumerate() {
            let mut up_connections = 0;

            for (x, &color) in row.iter().enumerate() {
                if color != 0 {
                    if let Some(&up_color) =
                        self.cells.get(y.wrapping_sub(1)).and_then(|r| r.get(x))
                    {
                        if up_color != 0 {
                            up_connections += 1;
                        }
                    }

                    continue;
                }

                if up_connections % 2 == 1 {
                    updated_cells[y][x] = 0xFFFFFF;
                }
            }
        }

        self.cells = updated_cells;

        Ok(())
    }

    pub fn count(&self) -> u32 {
        self.cells
            .iter()
            .flatten()
            .filter(|&&color| color != 0)
            .count() as u32
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            for &color in row {
                let c = match color {
                    0 => '.',
                    _ => '#',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
