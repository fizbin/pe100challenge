use std::cmp::max;
use std::convert::TryFrom;

fn main() {
    let mut bestval = 0;
    for ndigits in 1..=4u8 {
        for n in k_digit_prefix(ndigits, bestval)..10_u64.pow(ndigits.into()) {
            bestval = max(bestval, find_pandigital_multiple(n));
        }
    }
    println!("{}", bestval);
}

fn find_pandigital_multiple(n: u64) -> u64 {
    // return 0 on "does not make a pandigital multiple"
    let mut digits = [false; 10];
    digits[0] = true;
    let mut total_digits = 0;
    let mut multiplier = 0;
    let mut retval = 0;
    while total_digits < 9 {
        multiplier += 1;
        let mut checkval = n * multiplier;
        total_digits += how_many_digits(checkval);
        retval = concat_number(retval, checkval);
        while checkval > 0 {
            let digit = usize::from(u8::try_from(checkval % 10).unwrap());
            if digits[digit] {
                return 0;
            }
            digits[digit] = true;
            checkval /= 10;
        }
    }
    retval
}

fn concat_number(a: u64, b: u64) -> u64 {
    a * 10_u64.pow(how_many_digits(b).into()) + b
}

fn k_digit_prefix(k: u8, n: u64) -> u64 {
    let mut ndigits = how_many_digits(n);
    let mut retval = n;
    while ndigits > k {
        retval /= 10;
        ndigits -= 1;
    }
    retval
}

fn how_many_digits(n: u64) -> u8 {
    let mut pow_10 = 10;
    let mut digits = 1;

    while n >= pow_10 {
        pow_10 *= 10;
        digits += 1;
    }
    digits
}
