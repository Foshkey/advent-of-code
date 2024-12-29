use rayon::prelude::*;

use crate::number_generator::gen;

pub struct SequenceFinder {
    length: usize,
    tickers: Vec<Vec<u8>>,
    changes: Vec<Vec<i8>>,
}

impl SequenceFinder {
    pub fn new(input: &str, length: usize) -> Self {
        let mut changes = Vec::new();
        let tickers = input
            .lines()
            .map(|line| {
                let mut secret = line.parse().unwrap();
                let mut change = Vec::new();
                let ticker = (0..length)
                    .map(|_| {
                        let new_secret = gen(secret);
                        let bananas = (new_secret % 10) as u8;
                        change.push(bananas as i8 - (secret % 10) as i8);
                        secret = new_secret;
                        bananas
                    })
                    .collect();
                changes.push(change);
                ticker
            })
            .collect();

        SequenceFinder {
            length,
            tickers,
            changes,
        }
    }

    pub fn find_max_bananas(&self) -> usize {
        // Generate sequences and check in parallel
        (-9..=9)
            .into_par_iter()
            .flat_map(|a| (-9..=9).into_par_iter().map(move |b| (a, b)))
            .flat_map(|(a, b)| (-9..=9).into_par_iter().map(move |c| (a, b, c)))
            .flat_map(|(a, b, c)| (-9..=9).into_par_iter().map(move |d| (a, b, c, d)))
            .map(|(a, b, c, d)| self.get_bananas(&[a, b, c, d]))
            .reduce(|| 0, usize::max)
    }

    fn get_bananas(&self, sequence: &[i8; 4]) -> usize {
        self.changes
            .iter()
            .enumerate()
            .filter_map(|(index, changes)| {
                (0..self.length - 3)
                    .find(|&i| sequence == &changes[i..i + 4])
                    .map(|i| self.tickers[index][i + 3] as usize)
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence() {
        let sequence_finder = SequenceFinder::new("123", 9);
        let expected_ticker = vec![0, 6, 5, 4, 4, 6, 4, 4, 2];
        let expected_changes = vec![-3, 6, -1, -1, 0, 2, -2, 0, -2];

        assert_eq!(expected_ticker, *sequence_finder.tickers.first().unwrap());
        assert_eq!(expected_changes, *sequence_finder.changes.first().unwrap());
    }

    #[test]
    fn test_get_bananas() {
        let sequence_finder = SequenceFinder::new(
            "\
1
2
3
2024",
            2000,
        );
        let sequence = [-2, 1, -1, 3];

        assert_eq!(23, sequence_finder.get_bananas(&sequence));
    }
}
