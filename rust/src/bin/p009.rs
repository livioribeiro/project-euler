//! A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
//!
//! a^2 + b^2 = c^2
//!
//! For example, 3^2 + 4^2 = 9 + 16 = 25 = 52.
//!
//! There exists exactly one Pythagorean triplet for which a + b + c = 1000.
//! Find the product abc.

fn main() {
    let mut break_it = false;
    for n in 1u32..1000 {
        for m in (n + 1)..1000 {
            // https://en.wikipedia.org/wiki/Pythagorean_triple#Generating_a_triple
            // Euclid's formula
            let (b, a, c) = (m.pow(2) - n.pow(2), 2 * m * n, m.pow(2) + n.pow(2));

            if a.pow(2) + b.pow(2) == c.pow(2) && a + b + c == 1000 {
               println!("{}^2 + {}^2 == {}^2 == {} + {} == {}", a, b, c, a.pow(2), b.pow(2), c.pow(2));
               println!("{} + {} + {} == 1000", a, b, c);
               println!("{} * {} * {} == {}", a, b, c, a * b * c);

               break_it = true;
               break
           }
        }
        if break_it {break}
    }
}
