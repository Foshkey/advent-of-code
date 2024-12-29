use std::collections::HashMap;

use crate::number_generator::gen;

pub fn find_max_bananas(input: &str, length: usize) -> usize {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .flat_map(|secret| get_sequence_map(secret, length))
        .fold(HashMap::new(), |mut acc, (key, value)| {
            *acc.entry(key).or_insert(0) += value;
            acc
        })
        .values()
        .max()
        .cloned()
        .unwrap()
}

fn get_sequence_map(secret: usize, length: usize) -> HashMap<[i8; 4], usize> {
    let mut changes = Vec::new();
    let mut secret = secret;
    let ticker: Vec<_> = (0..length)
        .map(|_| {
            let new_secret = gen(secret);
            let bananas = (new_secret % 10) as u8;
            changes.push(bananas as i8 - (secret % 10) as i8);
            secret = new_secret;
            bananas
        })
        .collect();

    ticker
        .iter()
        .enumerate()
        .skip(3)
        .rev()
        .map(|(index, &bananas)| {
            let mut arr = [0; 4];
            arr.copy_from_slice(&changes[index - 3..=index]);
            (arr, bananas as usize)
        })
        .collect()
}
