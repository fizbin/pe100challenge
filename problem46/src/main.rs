use bitvec::prelude::*;

const LIMIT: usize = 1000000;
const LIMITSQR: usize = 1000;

fn main() {
    let mut primes = bitvec![LocalBits, u8; 1; LIMIT];
    primes.set(0, false);
    primes.set(1, false);
    for idx in 2..(LIMIT / 2) {
        if primes[idx] {
            let mut mulidx = usize::from(2 * idx);
            while mulidx < LIMIT {
                primes.set(mulidx, false);
                mulidx += idx;
            }
        }
    }
    // At this point, primes[p] is true iff p is prime
    let mut scratch = primes.clone();
    for n in 1..LIMIT / 2 {
        scratch.set(2 * n, true);
    }
    scratch.set(0, true);
    scratch.set(1, true);
    // scratch will be true if it's covered by the conjecture, or
    // is something the conjecture doesn't care about, like even
    // numbers or prime numbers or 1. The first "false" will be the
    // thing that we want to contradict the conjecture.
    println!("spot A");
    let mut shift2primes = primes.clone();
    shift2primes.shift_right(2);
    println!("spot B");
    for n in 1..LIMITSQR {
        let dblsq = 2 * n * n;
        if dblsq >= LIMIT {
            break;
        }
        let dblsqdiv8 = dblsq / 8;
        let dblsqrem8 = dblsq % 8;
        let shiftedprimes = match dblsqrem8 {
            0 => &primes,
            2 => &shift2primes,
            r => panic!("Can't happen; got remainder {}", r),
        };
        scratch.as_mut_raw_slice()[dblsqdiv8..]
            .iter_mut()
            .zip(shiftedprimes.as_raw_slice())
            .for_each(|(s, p)| {
                *s |= p;
            });
        match scratch.first_zero() {
            None => (),
            Some(x) => {
                if x <= dblsq + 2 {
                    break;
                }
            }
        }
    }
    println!("{:?}", scratch.first_zero());
}
