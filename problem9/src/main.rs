fn main() {
    println!("{}", problemloop());
}

fn problemloop() -> u64 {
    let mut m = 1;
    loop {
        for n in 1..m {
            let a = m*m - n*n;
            let b = 2*m*n;
            let c = m*m + n*n;

            if 1000 % (a + b + c) == 0 {
                let k = 1000 / (a + b + c);
                return (k*a)*(k*b)*(k*c);
            }
        }
        m += 1;
    }
}
