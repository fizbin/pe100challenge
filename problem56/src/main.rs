use num_bigint::BigUint;
use num_traits::Zero;
use std::cmp::max;
use std::convert::From;
use std::convert::TryFrom;
use std::iter::Iterator;

fn main() {
    let mut summax = 0;
    for a in 1..100 {
        for b in 1..100 {
            summax = max(powsum(a, b), summax);
        }
    }
    println!("{}", summax);
}

fn powsum(a: u32, b: u32) -> u32 {
    let mut full = BigUint::from(a).pow(b);
    let mut digsum = 0u32;
    while !full.is_zero() {
        let dig: u32 = full
            .iter_u32_digits()
            .reduce(|acc, d| acc % 10 + 6 * (d % 10))
            .unwrap()
            % 10;
        digsum += dig;
        // Slower ways to get the last digit:
        //  u32::from(full.to_radix_le(10)[0]);
        //  full.clone() % 10u32;
        //  &full % 10u32;
        full /= 10u32;
    }
    u32::try_from(digsum).unwrap()
}
