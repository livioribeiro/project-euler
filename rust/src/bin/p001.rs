//! If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
//! The sum of these multiples is 23.
//!
//! Find the sum of all the multiples of 3 or 5 below 1000.

const FACTORS: [u32; 7] = [0, 3, 5, 6, 9, 10, 12];

fn main() {
    let mut total = 0;

    // 990 is the last multiple of 15 below 1000
    for i in 1..(990 / 15 + 1) {
        let cur = i * 15;
        total += FACTORS.iter().fold(0, |acc, factor| acc + cur - factor);
    }

    // multiples of 3 and 5 remaining
    total += 993 + 995 + 996 + 999;

    println!("{}", total);
}

// // shorter solution
// fn main() {
//     let total = (1..(1000 / 3 + 1)).fold(0, |acc, i| acc + i * 3)
//         + (1..(1000 / 5)).fold(0, |acc, i| acc + i * 5) // 1000 is multiple of 5, so we don't include
//         - (1..(1000 / 15 + 1)).fold(0, |acc, i| acc + i * 15);
//
//     println!("{}", total);
// }
