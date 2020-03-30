fn main() {
    let mut possibilities = [[0u64; 8]; 500]; // 500 is overkill
    let denoms: [usize; 8] = [1, 2, 5, 10, 20, 50, 100, 200];
    for a in 1..=200 {
        for denom_idx in 0..8 {
            if denoms[denom_idx] < a {
                let prev = &possibilities[a - denoms[denom_idx]];
                possibilities[a][denom_idx] = prev[0..=denom_idx].iter().sum();
            }
            if denoms[denom_idx] == a {
                possibilities[a][denom_idx] = 1;
            }
        }
    }
    println!("{}", possibilities[200].iter().sum::<u64>());
}
