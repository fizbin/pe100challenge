fn main() {
    let sieve = get_prime_sieve(1000000);
    let mut n = 300;
    while n + 3 < 1000000
        && (sieve[n] != 4 || sieve[n + 1] != 4 || sieve[n + 2] != 4 || sieve[n + 3] != 4)
    {
        n = n + 1;
    }
    println!("{}", n);
}

fn get_prime_sieve(sieve_size: usize) -> Vec<u8> {
    let mut sieve: Vec<u8> = vec![0; sieve_size];
    for factor in 2..(sieve_size / 2) {
        if sieve[factor] == 0 {
            sieve[factor] = 1;
            let mut n = 2 * factor;
            while n < sieve_size {
                sieve[n] += 1;
                n += factor;
            }
        }
    }
    return sieve;
}
