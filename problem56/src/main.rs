use num_bigint::BigUint;
use num_traits::Zero;
use std::cmp::max;
use std::convert::TryFrom;

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
    let mut digsum = BigUint::zero();
    while !full.is_zero() {
        digsum += full.clone() % 10u32;
        full /= 10u32;
    }
    u32::try_from(digsum).unwrap()
}

