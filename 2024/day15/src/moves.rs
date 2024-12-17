use std::collections::VecDeque;

pub struct Moves {
    moves: VecDeque<(isize, isize)>,
}

impl Iterator for Moves {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        self.moves.pop_front()
    }
}

impl From<&str> for Moves {
    fn from(s: &str) -> Self {
        Moves {
            moves: s
                .chars()
                .filter_map(|c| match c {
                    '<' => Some((0, -1)),
                    '^' => Some((-1, 0)),
                    '>' => Some((0, 1)),
                    'v' => Some((1, 0)),
                    _ => None,
                })
                .collect(),
        }
    }
}
