//! The sum of the squares of the first ten natural numbers is,
//! 1^2 + 2^2 + ... + 10^2 = 385
//!
//! The square of the sum of the first ten natural numbers is,
//! (1 + 2 + ... + 10)^2 = 552 = 3025
//!
//! Hence the difference between the sum of the squares of the first ten natural numbers and the
//! square of the sum is 3025 − 385 = 2640.
//!
//! Find the difference between the sum of the squares of the first one hundred natural numbers and
//! the square of the sum.

fn main() {
    println!("{}", sum_square_difference(100));
}

fn sum_square_difference(up_to: u32) -> u32 {
    (1u32..(up_to + 1)).fold(0, |acc, e| acc + e).pow(2) - (1u32..(up_to + 1)).fold(0, |acc, e| acc + e.pow(2))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_up_to_10() {
        assert_eq!(2640, super::sum_square_difference(10));
    }
}
