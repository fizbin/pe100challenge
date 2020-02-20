fn main() {
    println!("{}", (1 .. 21).fold(1, lcm));
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a < b {
        return gcd(b, a);
    }
    while b > 1 {
        let c = a % b;
        if c == 0 {
            return b;
        }
        a = b;
        b = c;
    }
    b
}

fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}
