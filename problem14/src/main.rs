const million: u64 = 1000000;

fn main() {
    println!("{}", find_longest_collatz_start());
}

fn find_longest_collatz_start() -> u64 {
    let mut vec_holder: Vec<u64> = vec![0; million as usize];
    let vec = vec_holder.as_mut_slice();

    vec[1] = 1;
    for x in 1..million {
        if vec[x as usize] > 0 {
            continue;
        }
        let mut y = x;
        let mut steps = 0;
        while y >= million || vec[y as usize] == 0 {
            if y % 2 == 0 {
                y /= 2;
            } else {
                y = 3 * y + 1;
            }
            steps += 1;
        }
        vec[x as usize] = steps + vec[y as usize];
    }
    (1..million)
        .max_by(|x, y| vec[*x as usize].cmp(&vec[*y as usize]))
        .unwrap()
}
