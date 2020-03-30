fn main() {
    let mut sum = 0;
    for n in 10..354294_u32 {
        if n == digsum(n) {
            sum += n;
        }
    }
    println!("{}", sum);
}

fn digsum(val: u32) -> u32 {
    let mut working_val = val;
    let mut sum = 0;
    while working_val > 0 {
        sum += (working_val % 10).pow(5);
        working_val /= 10;
    }
    sum
}
