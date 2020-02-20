fn main() {
    let sum: u64 = (1 .. 101).fold(0, |a, b| a + b);
    let sqsum: u64 = (1 .. 101).fold(0, |a, b| a + b*b);
    println!("{}", sum*sum - sqsum);
}
