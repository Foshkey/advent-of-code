pub fn generate_number(secret: usize, iteration: usize) -> usize {
    (0..iteration).fold(secret, |secret, _| gen(secret))
}

pub fn gen(secret: usize) -> usize {
    let secret = mix_prune(secret * 64, secret);
    let secret = mix_prune(secret / 32, secret);
    mix_prune(secret * 2048, secret)
}

fn mix_prune(num: usize, secret: usize) -> usize {
    (num ^ secret) % 16777216
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen() {
        let mut secret = 123;
        let expected_list = [
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];

        for expected in expected_list {
            secret = gen(secret);
            assert_eq!(expected, secret);
        }
    }
}
