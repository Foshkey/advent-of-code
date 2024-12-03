const INPUT: &str = include_str!("input.txt");

fn sum_muls(input: &str, parse_do_dont: bool) -> usize {
    let mut sum = 0;
    let mut i = 0;
    let mut enabled = true;

    // This is using the cursor method, scanning along the input for key words
    while (0..input.len()).contains(&i) {
        // Check for mul(...)
        if input[i..].starts_with("mul(") {
            i += 4;
            let (result, cursor) = parse_mul(&input[i..]);

            if enabled {
                sum += result;
            }

            i += cursor;
            continue;
        }

        // Check for do() or don't()
        if parse_do_dont {
            if input[i..].starts_with("do()") {
                enabled = true;
                i += 4;
                continue;
            } else if input[i..].starts_with("don't()") {
                enabled = false;
                i += 7;
                continue;
            }
        }

        // Increment the cursor
        i += 1;
    }

    sum
}

fn parse_mul(input: &str) -> (usize, usize) {
    let mut a = None;
    let mut b_index = None;

    // Note that we're starting with the first number, we're already past mul(
    for (i, c) in input.char_indices() {
        // Is it a digit? Continue on
        if c.is_ascii_digit() {
            continue;

        // If it's a comma (and we didn't already encounter one), save the slice and continue with b
        } else if c == ',' && b_index.is_none() {
            a = Some(&input[0..i]);
            b_index = Some(i + 1);

        // Final state - parse a and b, multiply, and return with result & cursor
        } else if c == ')' {
            let Some(a) = a else { return (0, i) };
            let Some(b_index) = b_index else {
                return (0, i);
            };
            let result = a.parse::<usize>().unwrap() * input[b_index..i].parse::<usize>().unwrap();
            return (result, i + 1);

        // If we encounter anything else, bail with cursor
        } else {
            return (0, i);
        }
    }

    (0, 0)
}

fn part_1(input: &str) -> usize {
    sum_muls(input, false)
}

fn part_2(input: &str) -> usize {
    sum_muls(input, true)
}

fn main() {
    println!("Part 1: {:?}", part_1(INPUT));
    println!("Part 2: {:?}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const EXAMPLE_2: &str = include_str!("example_2.txt");

    #[test]
    fn test_example_part_1() {
        let result = part_1(EXAMPLE);
        assert_eq!(result, 161);
    }

    #[test]
    fn test_example_part_2() {
        let result = part_2(EXAMPLE_2);
        assert_eq!(result, 48);
    }
}
