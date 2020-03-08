use itertools::Itertools;

const SIEVE_WIDTH: usize = 2500;
const SIEVE_HEIGHT: usize = 20000;

fn main() {
    println!("{:?}", find_answer());
}

fn find_answer() -> Result<u64, &'static str> {
    let mut vec_holder: Vec<u8> = vec![0; SIEVE_HEIGHT * SIEVE_WIDTH];
    let mut vec_holder2: Vec<_> = vec_holder
        .as_mut_slice()
        .chunks_mut(SIEVE_WIDTH as usize)
        .collect();
    let sieve: &mut [&mut [u8]] = vec_holder2.as_mut_slice();
    let mut primes = [0u32; SIEVE_WIDTH];
    let mut curr_prod = [1u32; SIEVE_HEIGHT];

    let mut n_primes = 0;
    for current_num in 2..SIEVE_HEIGHT {
        let mut prime_factor_n = 0;
        if curr_prod[current_num] < current_num as u32 {
            let prime_factor;
            if curr_prod[current_num] == 1 {
                // New prime!
                primes[n_primes] = current_num as u32;
                prime_factor_n = n_primes;
                prime_factor = current_num as u32;
                n_primes += 1;
            } else {
                // power of a prime
                prime_factor = (current_num as u32) / curr_prod[current_num];
                for pnum in 0..n_primes + 1 {
                    if primes[pnum] == prime_factor {
                        prime_factor_n = pnum;
                        break;
                    }
                    if primes[pnum] == 0 {
                        panic!(
                            "Didn't find expected prime {} for current {} and factor {}",
                            prime_factor, current_num, curr_prod[current_num]
                        )
                    }
                }
            }
            // now go along filling in the rest of the sieve
            let mut ndx = current_num;
            while ndx < SIEVE_HEIGHT {
                sieve[ndx][prime_factor_n] += 1;
                curr_prod[ndx] *= prime_factor as u32;
                ndx += current_num;
            }
        }
        // now check for 500 factors
        // The factor count for the next triangular number will be...
        let n_factors = sieve[current_num]
            .iter()
            .zip(sieve[current_num - 1].iter())
            .map(|(a, b)| (a + b) as u32)
            .fold1(|acc, exptot| acc * (exptot + 1))
            .expect("broken fold");
        if n_factors > 500 {
            return Ok(((current_num * (current_num - 1)) / 2) as u64);
        }
    }
    Err("Not found")
}
