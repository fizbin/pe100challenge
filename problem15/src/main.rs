fn main() {
    let mut mul : u64 = 40;
    let mut div : u64 = 20;
    let mut ans : u64 = 1;

    while mul > 20 || div > 1 {
        while div > 1 && ans % div == 0 {
            ans /= div;
            div -= 1;
        }
        ans *= mul;
        mul -= 1;
    }
    println!("{}", ans);
}
