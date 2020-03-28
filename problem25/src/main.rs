use rug::{Assign, Integer};

fn main() {
    let mut fib1 = Integer::from(1);
    let mut fib2 = Integer::from(1);
    let mut tmp = Integer::new();
    let limit = Integer::from(Integer::i_pow_u(10,999));
    let mut n : u32 = 1;

    loop {
        tmp.assign(&fib2);
        tmp += &fib1;
        fib1.assign(&fib2);
        fib2.assign(&tmp);
        n += 1;
        if fib1 > limit {
            break;
        }
    }
    println!("{}", n);
}
