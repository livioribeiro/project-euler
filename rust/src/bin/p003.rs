//! The prime factors of 13195 are 5, 7, 13 and 29.
//!
//! What is the largest prime factor of the number 600851475143 ?

const INPUT: u64 = 600851475143;

fn main() {
    let mut i = (INPUT as f64).sqrt().ceil() as u64;

    while i > 1 {
        if INPUT % i == 0 && i.is_prime() {
            println!("{}", i);
            break
        }

        i -= 2;
    }
}

trait IsPrime {
    fn is_prime(&self) -> bool;
}

impl IsPrime for u64 {
    fn is_prime(&self) -> bool {
        if *self <= 2 {
            return true
        }

        if self % 2 == 0 {
            return false
        }

        let mut i = 3;
        let upper_bound = (*self as f64).sqrt().ceil() as u64;

        while i <= upper_bound {
            if self % i == 0 {
                return false
            }
            i += 2;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::IsPrime;

    fn slow_but_accurate(num: u64) -> bool {
        if num == 1 || num == 2 {
            return true
        }

        if num % 2 == 0 {
            return false
        }

        for i in 2..((num / 2) as u64 + 1) {
            if num as u64 % i == 0 {
                return false
            }
        }

        return true
    }

    #[test]
    fn up_to_10000() {
        for i in 2..10000 {
            assert_eq!(slow_but_accurate(i), i.is_prime());
        }
    }
}
