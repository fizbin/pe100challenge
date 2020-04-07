use std::vec;

fn main() {
    let mut answer_count = vec![0u32; 1001];

    for n in 1..500 {
        let mut m = n + 1;
        while 2 * m * (m + n) <= 1000 {
            if gcd(m, n) == 1 {
                let mut k = 1;
                while k * 2 * m * (m + n) <= 1000 {
                    let idx = k * 2 * m * (m + n);
                    answer_count[idx] += 1;
                    k += 1;
                }
            }
            m += 2;
        }
    }
    println!(
        "{:?}",
        (0..=1000)
            .into_iter()
            .max_by(|x, y| answer_count[*x].cmp(&answer_count[*y]))
    );
}

fn gcd(a: usize, b: usize) -> usize {
    if a < b {
        return gcd(b, a);
    }
    if a % b == 0 {
        return b;
    }
    gcd(b, a % b)
}
