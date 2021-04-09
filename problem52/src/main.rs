fn main() {
    let mut base = 10u32;
    while !do_loop(base) {
        base *= 10;
    }
}

//; Assumes that base is a power of 10
fn do_loop(base: u32) -> bool {
    let end = (10 * base) / 6;
    for n in ((base + 8)..end).step_by(9) {
        if digit_sig(n) == digit_sig(n * 6) {
            let goal = digit_sig(n);
            if goal == digit_sig(n * 2)
                && goal == digit_sig(n * 3)
                && goal == digit_sig(n * 4)
                && goal == digit_sig(n * 5)
            {
                println!("{}", n);
                return true;
            }
        }
    }
    false
}

fn digit_sig(n: u32) -> [u8; 10] {
    let mut ans = [0; 10];
    if n == 0 {
        ans[0] = 1;
    } else {
        let mut m = n;
        while m > 0 {
            ans[(m % 10) as usize] += 1;
            m /= 10;
        }
    }
    ans
}
