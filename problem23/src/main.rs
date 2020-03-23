use bitvec::prelude::*;
use num::integer::Integer;
use std::vec::Vec;

const LIMIT: usize = 28200;

fn main() {
    let mut abundants = bitvec![Local, u8; 0; LIMIT];
    let mut abundants_vec = Vec::new();
    for idx in 0..LIMIT {
        let idx_u32 = idx as u32;
        if is_abundant(idx_u32) {
            abundants.set(idx, true);
            abundants_vec.push(idx);
        }
    }
    let mut sums = bitvec![Local, u8; 0; LIMIT];

    // switch true<->false to do this with bitvec manipulation or not
    if true {
        // 0.07s

        let abundants_bitvecs: Vec<BitVec<_, _>> = (0..8).map(|x| abundants.clone() >> x).collect();

        for abundant in &abundants_vec {
            let (div8, rem8) = abundant.div_rem(&8);

            // I really wish I had some sort of |= for BitSlices here that
            // worked like this:
            //     sums[8*div8..] |= &(abundants_bitvecs[rem8][..]);
            // but apparently this is how you're supposed to do it:

            sums.as_mut_slice()[div8..]
                .iter_mut()
                .zip(abundants_bitvecs[rem8].as_slice())
                .for_each(|(u1, u2)| {
                    *u1 |= u2;
                });
        }
    } else {
        // 0.23s
        for a1 in &abundants_vec {
            for a2 in &abundants_vec {
                let sum = a1 + a2;
                if sum < LIMIT {
                    sums.set(sum, true);
                }
            }
        }
    }
    let mut sum_of_unreachable = 0;
    for test in 0..LIMIT {
        if !sums[test] {
            sum_of_unreachable += test;
        }
    }
    println!("{}", sum_of_unreachable);
}

fn is_abundant(n: u32) -> bool {
    if n < 1 {
        return false;
    }
    let mut divsum = 1;
    let mut divisor = 2;
    while divisor * divisor < n {
        if n % divisor == 0 {
            divsum += divisor;
            divsum += n / divisor;
        }
        divisor += 1;
    }
    if divisor * divisor == n {
        divsum += divisor;
    }
    divsum > n
}