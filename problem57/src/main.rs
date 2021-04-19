use num_bigint::BigUint;
use num_traits::Zero;

fn main() {
    let mut u = BigUint::from(3u32);
    let mut v = BigUint::from(2u32);
    let mut count = 0;
    for _ in 1..=1000 {
        let u2 = 2u32*&v + &u;
        let v2 = &u + &v;
        u = u2;
        v = v2;
        if dig_count(&u) > dig_count(&v) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn dig_count(src: &BigUint) -> u32 {
    let mut n = src.clone();
    if !n.is_zero() {
        let mut count = 0;
        while !n.is_zero() {
            count += 1;
            n /= 10u32;
        }
        count
    }
    else {
        1
    }
}
