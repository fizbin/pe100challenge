fn main() {
    let sieve = get_prime_sieve(100000);
    let prime_list = sieve_to_prime_list(&sieve);
    let mut cornercount: u64 = 1;
    let mut primecount: u64 = 0;
    let mut val_to_check: u64 = 1;
    let mut amt_to_add: u64 = 2;
    loop {
        if check_primality(val_to_check + amt_to_add, &prime_list) {
            primecount += 1;
        }
        if check_primality(val_to_check + 2 * amt_to_add, &prime_list) {
            primecount += 1;
        }
        if check_primality(val_to_check + 3 * amt_to_add, &prime_list) {
            primecount += 1;
        }
        if check_primality(val_to_check + 4 * amt_to_add, &prime_list) {
            primecount += 1;
        }
        val_to_check += 4 * amt_to_add;
        amt_to_add += 2;
        cornercount += 4;
        //println!("{} {}", primecount, cornercount);
        if 10 * primecount < cornercount {
            println!("{}", amt_to_add - 1);
            break;
        }
    }
}

fn check_primality(n: u64, primes: &Vec<usize>) -> bool {
    if n < 2 {
        return false;
    }
    for pu in primes {
        let p = *pu as u64;
        if p * p > n {
            return true;
        }
        if n % p == 0 {
            return false;
        }
    }
    panic!("Can't check primality of {}", n);
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

fn sieve_to_prime_list(sieve: &Vec<bool>) -> Vec<usize> {
    let mut retval = Vec::new();
    for idx in 2..sieve.len() {
        if sieve[idx] {
            retval.push(idx);
        }
    }
    retval.shrink_to_fit();
    return retval;
}
