fn main() {
    println!("{}", problem_loop());
}

fn problem_loop() -> u64 {
    // As a separate function to allow easy return from inside a nested loop
    
    // iterate so that we can bail as early as possible
    
    let mut factorsum = 999 + 999;
    let mut limit = 0; // aka "best palindrome so far"
    loop {
        let mut factordiff = factorsum % 2;
        while factorsum + factordiff < 2000 {
            let a = (factorsum + factordiff) / 2;
            let b = (factorsum - factordiff) / 2;
            let prod = a * b;
            if prod < limit {
                if factordiff < 2 {
                    return limit;
                }
                break; // we're done with this sum
            }
            if is_palindrome(prod) {
                limit = prod;
            }
            factordiff += 1;
        }
        factorsum -= 1;
    }
}

fn is_palindrome(test: u64) -> bool {
    let teststr = test.to_string();
    let myslice = teststr.as_bytes();  // Only ASCII, so ok
    let mut a = 0;
    let mut b = myslice.len() - 1;
    while a < b {
        if myslice[a] != myslice[b] {
            return false;
        }
        a += 1;
        b -= 1;
    }
    true
}
