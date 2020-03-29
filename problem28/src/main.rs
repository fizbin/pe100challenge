fn main() {
    let mut sum: u64 = 1;
    let mut n: u64 = 1;
    let mut ndiff: u64 = 2;
    loop {
        n += ndiff;
        sum += n;
        n += ndiff;
        sum += n;
        n += ndiff;
        sum += n;
        n += ndiff;
        sum += n;
        ndiff += 2;
        if n >= 1001*1001 {
            break;
        }
    }
    println!("{}", sum);
}
