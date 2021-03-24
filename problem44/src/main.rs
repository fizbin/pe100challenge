use std::collections::BinaryHeap;

fn main() {
    let mut heapq = BinaryHeap::new();
    let ival: (u64, u64, u64) = (!4, 1, 1);
    heapq.push(ival);
    let mut retval;
    loop {
        match heapq.pop() {
            None => panic!("Can't happen"),
            Some((_, n, k)) => {
                let pn = n * (3 * n - 1) / 2;
                let pk = (n + k) * (3 * (n + k) - 1) / 2;
                retval = pk - pn;
                if is_pentagonal(retval) {
                    //println!("{} and {}", n, n+k);
                    let sump = pn + pk;
                    if is_pentagonal(sump) {
                        break;
                    }
                }
                let pk2 = (n + k + 1) * (3 * (n + k + 1) - 1) / 2;
                if n == 1 {
                    // add new k
                    heapq.push((!(pk2 - pn), n, k + 1))
                }
                let pn2 = (n + 1) * (3 * (n + 1) - 1) / 2;
                heapq.push((!(pk2 - pn2), n + 1, k));
            }
        }
    }
    println!("{}", retval);
}

fn is_pentagonal(n: u64) -> bool {
    // true iff (1 + 24*n) is the square of a number that is ~= 5 mod 6
    let tgt = 1 + 24*n;
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
