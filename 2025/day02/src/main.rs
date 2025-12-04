use crate::product_list::ProductList;

mod product_list;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part_1(input)?);
    println!("Part 2: {}", part_2(input)?);
    Ok(())
}

fn part_1(input: &str) -> Result<usize> {
    let product_list: ProductList = input.parse()?;
    product_list.get_sum_doubles()
}

fn part_2(input: &str) -> Result<usize> {
    let product_list: ProductList = input.parse()?;
    product_list.get_sum_multiples()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE).unwrap(), 1227775554);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE).unwrap(), 4174379265);
    }
}
