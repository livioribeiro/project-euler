//! A palindromic number reads the same both ways. The largest palindrome made from the product of
//! two 2-digit numbers is 9009 = 91 × 99.
//!
//! Find the largest palindrome made from the product of two 3-digit numbers.

fn main() {
    let mut largest = 0;
    let mut largest_factor1 = 0;
    let mut largest_factor2 = 0;

    for i in (101..1000).rev() {
        // anything multiplied by 10 ends in zero
        if i % 10 == 0 {
            continue
        }

        for j in (101..1000).rev() {
            let num = i * j;

            // continue if number ends in zero
            if j % 10 == 0 || num % 10 == 0 {
                continue
            }

            if is_palindrome(num) && num > largest {
                largest = num;
                largest_factor1 = i;
                largest_factor2 = j
            }
        }
    }

    println!("{} = {} * {}", largest, largest_factor1, largest_factor2);
}

fn get_level(num: u64) -> u16 {
    let mut level = 1;
    let mut divisor = 10;

    while num / divisor >= 1 {
        level += 1;
        divisor *= 10;
    }

    level
}

fn get_digit_at(pos: u16, num: u64) -> u16 {
    ((num % 10u64.pow(pos as u32)) / 10u64.pow(pos as u32 - 1)) as u16
}

fn is_palindrome(num: u64) -> bool {
    let mut i = get_level(num);
    let mut j = 1;

    while i > j {
        if get_digit_at(i, num) != get_digit_at(j, num) {
            return false
        }

        i -= 1;
        j += 1;
    }

    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn level_1() {
        assert_eq!(1, super::get_level(1));
        assert_eq!(1, super::get_level(2));
        assert_eq!(1, super::get_level(5));
        assert_eq!(1, super::get_level(9));
    }

    #[test]
    fn level_2() {
        assert_eq!(2, super::get_level(10));
        assert_eq!(2, super::get_level(21));
        assert_eq!(2, super::get_level(52));
        assert_eq!(2, super::get_level(93));
    }

    #[test]
    fn level_3() {
        assert_eq!(3, super::get_level(100));
        assert_eq!(3, super::get_level(201));
        assert_eq!(3, super::get_level(520));
        assert_eq!(3, super::get_level(932));
    }

    #[test]
    fn digit_at_1() {
        assert_eq!(5, super::get_digit_at(1, 12345));
    }

    #[test]
    fn digit_at_2() {
        assert_eq!(4, super::get_digit_at(2, 12345));
    }

    #[test]
    fn digit_at_3() {
        assert_eq!(3, super::get_digit_at(3, 12345));
    }

    #[test]
    fn digit_at_4() {
        assert_eq!(2, super::get_digit_at(4, 12345));
    }

    #[test]
    fn digit_at_5() {
        assert_eq!(1, super::get_digit_at(5, 12345));
    }

    #[test]
    fn is_palindrome_even_digits() {
        assert!(super::is_palindrome(123321));
        assert!(super::is_palindrome(1221));
        assert!(super::is_palindrome(11));
    }

    #[test]
    fn is_palindrome_odd_digits() {
        assert!(super::is_palindrome(12321));
        assert!(super::is_palindrome(121));
    }

    #[test]
    fn is_palindrome_one_digit() {
        assert!(super::is_palindrome(1));
    }

    #[test]
    fn is_not_palindrome() {
        assert!(!super::is_palindrome(123312));
        assert!(!super::is_palindrome(12312));
        assert!(!super::is_palindrome(1234));
        assert!(!super::is_palindrome(123));
        assert!(!super::is_palindrome(12));
    }
}
