fn main() {
    let mut sum = 0;
    for val in 0..1000 {
        if (val % 5 == 0) || (val % 3 == 0) {
            sum += val;
        }
    }
    println!("P1 anser: {}", sum);
}
