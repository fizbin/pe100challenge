fn main() {
    let mut val : u64 = 600851475143;
    println!("Problem 3 answer: {}",
            loop {
                let p = find_prime_factor(val);
                if p == val {
                    break val;
                }
                val /= p;
            });
}

fn find_prime_factor(x: u64) -> u64 {
    // Not super-efficient, but...
    if x % 2 == 0 {
        return 2;
    }
    if x % 3 == 0 {
        return 3;
    }
    let mut factor = 5;
    while factor * factor <= x {
        if x % factor == 0 {
            return factor;
        }
        if x % (factor + 2) == 0 {
            return factor + 2;
        }
        factor += 6;
    }
    return x; // if x is prime
}
