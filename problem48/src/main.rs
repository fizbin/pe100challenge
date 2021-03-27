const FIVE_ZEROS: u64 = 100000;
const TEN_ZEROS: u64 = FIVE_ZEROS * FIVE_ZEROS;

fn main() {
    let mut sum = 0;
    for n in 1..=1000 {
        sum += pow10dig(n, n);
    }
    println!("{}", sum % TEN_ZEROS);
}

fn pow10dig(base: u64, pow: u64) -> u64 {
    let ans = if pow == 0 {
        1
    } else if pow % 2 == 1 {
        mul10dig(base, pow10dig(base, pow - 1))
    } else {
        let p2 = pow10dig(base, pow / 2);
        mul10dig(p2, p2)
    };
    ans
}

fn mul10dig(a: u64, b: u64) -> u64 {
    let alow = a % FIVE_ZEROS;
    let blow = b % FIVE_ZEROS;
    let ahigh = a - alow;
    let bhigh = b - blow;
    return (alow * blow + ahigh * blow + alow * bhigh) % TEN_ZEROS;
}
