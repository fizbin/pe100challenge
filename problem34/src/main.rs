use lazy_static::lazy_static;
use std::vec::Vec;

lazy_static! {
    static ref FACTORIALS: Vec<u32> = (0u32..=9).map(|x| (1..=x).product()).collect();
}

fn main() {
    println!(
        "{}",
        (10..(7 * FACTORIALS[9]))
            .filter(|x| *x == digital_factorial_sum(*x))
            .sum::<u32>()
    );
}

fn digital_factorial_sum(mut n: u32) -> u32 {
    let mut sum = 0;
    while n > 0 {
        sum += FACTORIALS[(n % 10) as usize];
        n /= 10;
    }
    sum
}
