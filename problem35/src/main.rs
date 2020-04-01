fn main() {
    let sieve = get_prime_sieve(1000001);
    let mut n_circular = 0;
    for n in 1..10 {
        if check(n, 1, &sieve[..], 0) {
            n_circular += 1;
        }
    }
    for n in 10..100 {
        if check(n, 10, &sieve[..], 1) {
            n_circular += 1;
        }
    }
    for n in 100..1000 {
        if check(n, 100, &sieve[..], 2) {
            n_circular += 1;
        }
    }
    for n in 1000..10000 {
        if check(n, 1000, &sieve[..], 3) {
            n_circular += 1;
        }
    }
    for n in 10000..100000 {
        if check(n, 10000, &sieve[..], 4) {
            n_circular += 1;
        }
    }
    for n in 100000..1000000 {
        if check(n, 100000, &sieve[..], 5) {
            n_circular += 1;
        }
    }
    println!("{}", n_circular);
}

fn check(n: usize, scale: usize, sieve: &[bool], n_stages: u8) -> bool {
    if n_stages > 0 {
        sieve[n] && check(rotate(n, scale), scale, sieve, n_stages - 1)
    } else {
        sieve[n]
    }
}

fn rotate(n: usize, scale: usize) -> usize {
    scale * (n%10) + n/10
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
