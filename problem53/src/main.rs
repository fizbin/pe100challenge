const LIMIT: u32 = 1000000;

fn main() {
    let mut row = [None; 101];
    row[0] = Some(1);
    let mut retval = 0;
    for i in 1..=100 {
        retval += nextrow(&mut row, i);
    }
    println!("{}", retval);
}

fn nextrow(row: &mut [Option<u32>], n: usize) -> u32 {
    // returns the number of entries over LIMIT
    // on input, entries 0..n of row are used, and on output entries 0..=n are used
    let mut scratch = vec![None; n + 1];
    scratch[0] = Some(1);
    scratch[n] = Some(1);
    for i in 1..n {
        scratch[i] = row[i - 1]
            .and_then(|x| row[i].map(|y| x + y))
            .and_then(|s| if s > LIMIT { None } else { Some(s) });
    }
    let mut retval = 0;
    for i in 0..=n {
        row[i] = scratch[i];
        match row[i] {
            None => retval += 1,
            Some(_) => (),
        }
    }
    retval
}
