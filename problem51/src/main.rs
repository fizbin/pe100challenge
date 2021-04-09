use std::cmp::min;
use std::collections::HashMap;
use std::collections::BTreeSet;

fn main() {
    let primes = get_prime_sieve(1000000);
    let mut buckets = HashMap::new();
    for n in 2..1000000 {
        if primes[n] {
            let fullstr = n.to_string();
            for ch in fullstr.chars() {
                for sig in get_all_sigs(&fullstr.replace(ch, "*"), ch).into_iter() {
                    match buckets.get_mut(&sig) {
                        None => {
                            let mut newset = BTreeSet::new();
                            newset.insert(n);
                            buckets.insert(sig, newset);
                        }
                        Some(existing) => {
                            existing.insert(n);
                        }
                    }
                }
            }
        }
    }
    //println!("{:?}", buckets.get("56**3"));
    let mut found = usize::MAX;
    for (_, bucket) in buckets.into_iter() {
        if bucket.len() >= 8 {
            //println!("{:?}", bucket);
            match bucket.iter().next() {
                Some(&x) => {
                    found = min(x, found);
                }
                None => panic!("Can't happen"),
            }
        }
    }
    println!("{}", found);
}

fn get_all_sigs(sig: &str, digit: char) -> Vec<String> {
    if sig.matches('*').count() == 1 {
        return vec![String::from(sig)];
    }
    let mut ans = Vec::new();
    let mut working = String::from(sig);
    loop {
        match working.rfind('*') {
            None => break,
            Some(idx) => {
                ans.push(working.clone());
                working.truncate(idx);
                working.push(digit);
                working.push_str(&sig[idx+1..]);
            }
        }
    }
    ans
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
