/// Least common multiple vec of numbers
pub fn lcm(numbers: Vec<usize>) -> usize {
    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    numbers
        .into_iter()
        .reduce(|a, b| a * b / gcd(a, b))
        .unwrap_or(1)
}
