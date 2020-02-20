fn main() {
    // Use some tricky math. Well, a little tricky
    let mut sum = 0;
    let mut fibn = 2;
    let mut fibnm1 = 1;

    while fibn < 4000000 {
        let a = 3 * fibn + 2 * fibnm1;
        let b = 2 * fibn + fibnm1;
        sum += fibn;
        fibn = a;
        fibnm1 = b;
    }
    println!("Problem 2 answer: {}", sum);
}
