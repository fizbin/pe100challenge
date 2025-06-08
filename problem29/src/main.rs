use dashu::Integer;
use std::collections::HashSet;

fn main() {
    let mut values = HashSet::new();

    for a in 2..=100_i32 {
        for b in 2..=100_usize {
            values.insert(Integer::from(a).pow(b));
        }
    }
    println!("{}", values.len());
}
