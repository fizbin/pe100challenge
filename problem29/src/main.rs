use std::collections::HashSet;
use rug::Integer;


fn main() {
    let mut values = HashSet::new();

    for a in 2..=100_i32 {
        for b in 2..=100_u32 {
            values.insert(Integer::from(Integer::i_pow_u(a, b)));
        }
    }
    println!("{}", values.len());
}
