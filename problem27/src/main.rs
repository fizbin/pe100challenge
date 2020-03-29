fn main() {
    let sieve = get_prime_sieve(1000000);
    let mut max_prod: i32 = 0;
    let mut max_prime_count: i32 = 0;
    for a in -999..1000_i32 {
        for b in -1000..=1000_i32 {
            let mut n = 0;
            loop {
                let qval = n * n + a * n + b;
                if qval < 0 || !sieve[qval as usize] {
                    break;
                }
                n += 1;
            }
            if n > max_prime_count {
                max_prime_count = n;
                max_prod = a * b;
            }
        }
    }
    println!("{}", max_prod);
}

fn get_prime_sieve(sieve_size: usize) -> Vec<bool> {
    let mut sieve: Vec<bool> = vec![true; sieve_size];
    sieve[0] = false;
    sieve[1] = false;
    for factor in 2..(sieve_size / 2) {
        if sieve[factor] {
            let mut n = 2 * factor;
            while n < sieve_size {
                sieve[n] = false;
                n += factor;
            }
        }
    }
    return sieve;
}
