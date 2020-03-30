use std::collections::HashSet;

fn main() {
    let mut good_prods = HashSet::new();
    // We either have a one digit times a four digit, or a two-digit times a 3
    for a in 1..9u32 {
        for b in 1000..9999u32 {
            let prod = a*b;
            if prod > 1000 && prod < 10000 && check_product(a, b) {
                good_prods.insert(prod);
            }
        }
    }
    for a in 10..99u32 {
        for b in 100..999u32 {
            let prod = a*b;
            if prod > 1000 && prod < 10000 && check_product(a, b) {
                good_prods.insert(prod);
            }
        }
    }
    println!("{}", good_prods.iter().sum::<u32>());
}

fn check_product(a: u32, b: u32) -> bool {
    let mut digits = [false; 10];
    let mut working_vals = [a, b, a*b];

    for widx in 0..3 {
        while working_vals[widx] > 0 {
            let digit = (working_vals[widx] % 10) as usize;
            if digits[digit] { // already have this digit
                return false;
            }
            digits[digit] = true;
            working_vals[widx] /= 10;
        }
    }

    (!digits[0]) && digits[1..].iter().all(|x| *x)
}
