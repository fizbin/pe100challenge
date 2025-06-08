use dashu::{Integer, ibig};

fn main() {
    let mut fib1 = Integer::from(1);
    let mut fib2 = Integer::from(1);
    let mut tmp = ibig!(0);
    let limit = ibig!(10).pow(999);
    let mut n: u32 = 1;

    loop {
        tmp.clone_from(&fib2);
        tmp += &fib1;
        fib1.clone_from(&fib2);
        fib2.clone_from(&tmp);
        n += 1;
        if fib1 > limit {
            break;
        }
    }
    println!("{}", n);
}
