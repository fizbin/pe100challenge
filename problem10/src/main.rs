fn main() {
    println!("{}",
             get_prime_sieve(2000000).iter().enumerate().filter(|a| *a.1)
             .fold(0, |a, b| a + b.0));
}

fn get_prime_sieve(sieve_size : usize) -> Vec<bool> {
    let mut sieve : Vec<bool> = vec![true; sieve_size];
    sieve[0] = false;
    sieve[1] = false;
    for factor in 2..(sieve_size / 2) {
        if sieve[factor] {
            let mut n = 2*factor;
            while n < sieve_size {
                sieve[n] = false;
                n += factor;
            }
        }
    }
    return sieve;
}
