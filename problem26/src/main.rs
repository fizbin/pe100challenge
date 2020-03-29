fn main() {
    let mut max_len_divisor = 1_u32;
    let mut max_len = 0;
    for divisor in 1..1000 {
        let divlen = find_reciprocal_cycle_length(divisor);
        if divlen > max_len {
            max_len = divlen;
            max_len_divisor = divisor;
        }
    }
    println!("{}  (len {})", max_len_divisor, max_len);
}

fn find_reciprocal_cycle_length(divisor: u32) -> u32 {
    
    let mut n = 10 % divisor;
    for _ in 0..divisor {
        n *= 10;
        n %= divisor;
    }
    match n {
        0 => 0,
        _ => {
            let m = n;
            let mut retval = 1;
            n *= 10;
            n %= divisor;
            if n == 0 {
                return 0;
            }
            loop {
                if n == m {
                    break;
                }
                n *= 10;
                n %= divisor;
                retval += 1;
            }
            retval
        }
    }
}
