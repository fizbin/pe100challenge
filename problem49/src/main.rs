use std::collections::HashMap;

fn main() {
    let primes = get_prime_sieve(10000);
    let mut primesigs: HashMap<[u8; 4], Vec<u32>> = HashMap::new();
    for n in 1000..=9999u32 {
        if primes[n as usize] {
            let sig = get_digits(n);
            match primesigs.get_mut(&sig) {
                Some(target) => target.push(n),
                None => {
                    primesigs.insert(sig, vec![n]);
                }
            };
        }
    }
    for (_, val) in primesigs.iter() {
        if val.len() >= 3 {
            for (a, b, c) in combo3(&val) {
                if a + c == b * 2 {
                    if a != 1487 {
                        println!("{}{}{}", a, b, c);
                        return;
                    }
                }
            }
        }
    }
}

fn combo3<'veclife, T: Copy>(val: &'veclife Vec<T>) -> impl Iterator<Item = (T, T, T)> + 'veclife {
    struct Spot3<'a, S: Copy>(usize, usize, usize, &'a Vec<S>);
    impl<S: Copy> Iterator for Spot3<'_, S> {
        type Item = (S, S, S);
        fn next(&mut self) -> Option<Self::Item> {
            if self.2 + 1 < self.3.len() {
                self.2 += 1;
            } else if self.1 + 1 < self.2 {
                self.1 += 1;
                self.2 = self.1 + 1;
            } else if self.0 + 1 < self.1 {
                self.0 += 1;
                self.1 = self.0 + 1;
                self.2 = self.1 + 1;
            } else {
                return None;
            }
            Some((self.3[self.0], self.3[self.1], self.3[self.2]))
        }
    }
    // Yes, deliberately 0, 1, 1 - so that the first returned will be 0, 1, 2
    return Spot3(0, 1, 1, val);
}

fn get_digits(n: u32) -> [u8; 4] {
    let mut ret = [0, 0, 0, 0];
    ret[0] = ((n / 1000) % 10) as u8;
    ret[1] = ((n / 100) % 10) as u8;
    ret[2] = ((n / 10) % 10) as u8;
    ret[3] = (n % 10) as u8;
    ret.sort();
    ret
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
