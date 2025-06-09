use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct State {
    // type invariant that is maintained by the code:
    // the first `nprimes` of the pN values have real primes, all others
    // are 0. The pN that are not 0 are all distinct, and are ordered with
    // the smallest in p1 to the largest in p{nprimes}
    cost: u64, // always set to the sum of the pN values
    nprimes: u8,
    p1: u64,
    p2: u64,
    p3: u64,
    p4: u64,
    p5: u64,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Deliberately reverse order on cost comparison so that we get a min-heap
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.p1.cmp(&other.p1))
            .then_with(|| self.p2.cmp(&other.p2))
            .then_with(|| self.p3.cmp(&other.p3))
            .then_with(|| self.p4.cmp(&other.p4))
            .then_with(|| self.p5.cmp(&other.p5))
            .then_with(|| self.nprimes.cmp(&other.nprimes))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn check_done(state: &State) -> bool {
    return state.nprimes > 4;
}

fn add_successors(target: &mut BinaryHeap<State>, current: &State, primes: &Vec<u64>) {
    let mut prime_idx = 0_usize;
    let prime_limit = current.p1;
    while primes[prime_idx + 1] < prime_limit {
        prime_idx += 1;
        let p = primes[prime_idx];
        if (current.nprimes >= 1) && !check_pairwise_concat(p, current.p1, primes) {
            continue;
        }
        if (current.nprimes >= 2) && !check_pairwise_concat(p, current.p2, primes) {
            continue;
        }
        if (current.nprimes >= 3) && !check_pairwise_concat(p, current.p3, primes) {
            continue;
        }
        if (current.nprimes >= 4) && !check_pairwise_concat(p, current.p4, primes) {
            continue;
        }
        target.push(State {
            cost: current.cost + p,
            nprimes: current.nprimes + 1,
            p1: p,
            p2: current.p1,
            p3: current.p2,
            p4: current.p3,
            p5: current.p4,
        });
    }
}

fn main() {
    let sieve = get_prime_sieve(100000);
    let prime_list = sieve_to_prime_list(&sieve);
    let mut heap: BinaryHeap<State> = BinaryHeap::from(
        prime_list
            .iter()
            .map(|p| State {
                cost: *p,
                nprimes: 1,
                p1: *p,
                p2: 0,
                p3: 0,
                p4: 0,
                p5: 0,
            })
            .collect::<Vec<_>>(),
    );
    while let Some(st) = heap.pop() {
        // if st.cost % 1000 == 0 {
        //     println!("DBG: {} = {st:#?}", st.cost);
        // }
        if check_done(&st) {
            println!("{}", st.cost);
            break;
        }
        add_successors(&mut heap, &st, &prime_list);
    }
}

fn check_pairwise_concat(n1: u64, n2: u64, primes: &Vec<u64>) -> bool {
    let mut n1m = 10;
    let mut n2m = 10;
    while n1 >= n1m {
        n1m *= 10;
    }
    while n2 >= n2m {
        n2m *= 10;
    }
    return check_primality(n1 + n1m * n2, primes) && check_primality(n2 + n2m * n1, primes);
}

fn check_primality(n: u64, primes: &Vec<u64>) -> bool {
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

fn sieve_to_prime_list(sieve: &Vec<bool>) -> Vec<u64> {
    let mut retval = Vec::new();
    for idx in 2..sieve.len() {
        if sieve[idx] {
            retval.push(idx as u64);
        }
    }
    retval.shrink_to_fit();
    return retval;
}
