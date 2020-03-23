use std::vec::*;

fn main() {
    let mut digits: Vec<&str> = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut desired_n: u32 = 1000000;
    let mut sofar: Vec<&str> = vec![];

    while digits.len() > 0 {
        assert!(desired_n <= factorial(digits.len() as u8));
        let mut next_idx = 0;
        let next_inc = factorial((digits.len() - 1) as u8);
        while desired_n > next_inc {
            desired_n -= next_inc;
            next_idx += 1;
        }
        sofar.push(digits.remove(next_idx));
    }
    println!("{}", sofar.join(""));
}

fn factorial(n: u8) -> u32 {
    if n < 2 {
        return 1;
    }

    u32::from(n) * factorial(n - 1)
}
