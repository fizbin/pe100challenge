fn main() {
    let mut sieve_size = 50000;
    println!("{}", loop {
        let (success, val) = find_nth_prime(sieve_size, 10001);
        if success {
            break val;
        }
        sieve_size *= 2;
    });
}

fn find_nth_prime(sieve_size: usize, mut n: u64) -> (bool, u64) {
    let mut sieve : Vec<bool> = vec![true; sieve_size];
    sieve[0] = false;
    sieve[1] = false;
    let mut prev_p = 1;
    loop {
        let mut next_p = prev_p + 1;
        while (next_p < sieve_size) && !sieve[next_p] {
            next_p += 1;
        }
        if next_p >= sieve_size {
            return (false, 0);
        }
        n -= 1;
        if n == 0 {
            return (true, next_p as u64);
        }
        prev_p = next_p;
        let mut ndx = next_p;
        while ndx < sieve_size {
            sieve[ndx] = false;
            ndx += next_p;
        }
    }
}
