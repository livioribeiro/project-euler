const UP_TO: u64 = 2_000_000;

fn main() {
    let mut total = 2;
    let mut i = 3;

    while i <= UP_TO {
        if is_prime(i) {
            total += i;
        }

        i += 2;
    }

    println!("{}", total);
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
