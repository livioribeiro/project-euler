//! By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
//! 
//! What is the 10 001st prime number?

fn main() {
    let mut pos = 0;

    for i in 2..std::u64::MAX {
        if is_prime(i) {
            pos += 1;
            if pos == 10_001 {
                println!("{}", i);
                break
            }
        }
    }
}

fn is_prime(num: u64) -> bool {
    if num <= 2 {
        return true
    }

    if num % 2 == 0 {
        return false
    }

    let mut i = 3;
    let upper_bound = (num as f64).sqrt().ceil() as u64;

    while i <= upper_bound {
        if num % i == 0 {
            return false
        }
        i += 2;
    }

    true
}
