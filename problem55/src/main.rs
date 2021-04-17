use num_bigint::BigUint;
use num_traits::identities::Zero;

fn main() {
    let mut count = 0;
    for tnum in 1..10000u64 {
        if is_lychrel(BigUint::from(tnum)) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn is_lychrel(mut t: BigUint) -> bool {
    t += reverse(&t);
    for _ in 1..50 {
        let revt = reverse(&t);
        if t == revt {
            return false;
        }
        t += revt;
    }
    true
}

fn reverse(src: &BigUint) -> BigUint {
    //println!("{}", n);
    if !src.is_zero() {
        let mut n = src.clone();
        let mut retval = BigUint::zero();
        while !n.is_zero() {
            retval *= 10u64;
            retval += &n % 10u64;
            n /= 10u64;
        }
        retval
    } else {
        BigUint::zero()
    }
}
