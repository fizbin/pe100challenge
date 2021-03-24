fn main() {
    let mut n : u64 = 144;
    loop {
        let hn = n * (2*n - 1);
        if is_pentagonal(hn) {
            println!("{}", hn);
            break;
        }
        n += 1;
    }
}

fn is_pentagonal(n: u64) -> bool {
    // true iff (1 + 24*n) is the square of a number that is ~= 5 mod 6
    let tgt = 1 + 24 * n;
    let mut tst = 1;
    let mut tmp = tgt;
    while tmp > 1 {
        tst <<= 1;
        tmp >>= 2;
    }
    while tst != tmp && tst > 0 {
        tmp = tst;
        tst = ((tgt / tst) + tst) / 2;
    }
    (tst * tst == tgt) && (tst % 6 == 5)
}
