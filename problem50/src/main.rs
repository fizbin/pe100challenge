fn main() {
    let primes = get_prime_sieve(1000000);
    let mut maxsum = 0;
    let mut maxcount = 0;
    for minval in 2..=999999 {
        if primes[minval] {
            let mut sum = minval;
            let mut count = 1;
            for maxval in (minval + 1)..=999999 {
                if primes[maxval] {
                    sum += maxval;
                    count += 1;
                    if sum > 999999 {
                        break;
                    }
                    if sum < primes.len() && primes[sum] {
                        if count > maxcount {
                            maxcount = count;
                            maxsum = sum;
                        }
                    }
                }
            }
        }
    }
    println!("{}", maxsum);
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
    sieve
}
