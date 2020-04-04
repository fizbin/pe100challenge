use std::collections::HashSet;

fn main() {
    // first build up all the right-truncatable primes
    let mut rtprimes: HashSet<u64> = HashSet::new();
    rtprimes.insert(2);
    rtprimes.insert(3);
    rtprimes.insert(5);
    rtprimes.insert(7);
    let mut rtprimes_edge = rtprimes.clone();
    while !rtprimes_edge.is_empty() {
        let mut new_edge = HashSet::new();
        for prime in rtprimes_edge {
            for last_digit in &[1, 3, 7, 9] {
                let testval = 10 * prime + last_digit;
                if is_prime(testval) {
                    new_edge.insert(testval);
                    rtprimes.insert(testval);
                }
            }
        }
        rtprimes_edge = new_edge;
    }
    // now test the left-truncatableness of these values
    let ans: Vec<u64> = rtprimes
        .into_iter()
        .filter(|x| *x > 10 && is_lt_prime(*x))
        .collect();
    println!("{:?}", ans);
    println!("{}", ans.iter().sum::<u64>());
}

fn is_lt_prime(testval: u64) -> bool {
    // i.e. "can I truncate from the left"
    if !is_prime(testval) {
        return false;
    }
    if testval < 10 {
        // Needed to make recursion work; excluded above
        return true;
    }
    let mut pow_10 = 10;
    while pow_10 < testval {
        pow_10 *= 10;
    }
    is_lt_prime(testval % (pow_10 / 10))
}

fn is_prime(testval: u64) -> bool {
    if testval < 13 {
        match testval {
            2 | 3 | 5 | 7 | 11 => return true,
            _ => return false,
        }
    }
    if (testval % 2 == 0)
        || (testval % 3 == 0)
        || (testval % 5 == 0)
        || (testval % 7 == 0)
        || (testval % 11 == 0)
    {
        return false;
    }
    let mut factor_base = 12;
    while factor_base * factor_base < testval {
        if (testval % (factor_base + 1) == 0)
            || (testval % (factor_base + 5) == 0)
            || (testval % (factor_base + 7) == 0)
            || (testval % (factor_base + 11) == 0)
        {
            return false;
        }
        factor_base += 12;
    }
    true
}
