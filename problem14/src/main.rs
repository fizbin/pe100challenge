use std::time::Instant;

const MILLION: u64 = 1000000;

fn main() {
    let start = Instant::now();
    println!("{}", find_longest_collatz_start());
    let duration = start.elapsed();
    println!("Elapsed: {:?}", duration);
}

fn find_longest_collatz_start() -> u64 {
    let mut vec_holder: Vec<u64> = vec![0; MILLION as usize];
    let mut maxval = 1;
    let vec = vec_holder.as_mut_slice();

    vec[1] = 1;
    for x in 2..MILLION {
        let mut y = x;
        let mut steps = 0;
        while y >= MILLION || vec[y as usize] == 0 {
            if y % 2 == 0 {
                y /= 2;
            } else {
                y = 3 * y + 1;
            }
            steps += 1;
        }
        vec[x as usize] = steps + vec[y as usize];
        if vec[x as usize] > vec[maxval as usize] {
            maxval = x;
        }
    }
    maxval
}
