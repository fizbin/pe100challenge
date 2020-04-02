fn main() {
    let mut sum = 0;
    find_all_base2_palindromes(|x| {
        if check_base10_palindrome(x) {
            sum += x;
        }
    });
    println!("{}", sum);
}

fn find_all_base2_palindromes<F>(mut found_func: F)
where
    F: FnMut(u32),
{
    found_func(1);
    for n_inner_digits in 0u8..=18 {
        let high_added: u32 = 1 << (n_inner_digits + 1);
        let mut inner_func = |x| found_func(high_added + 2 * x + 1);
        base2_palindrome_loop(n_inner_digits, &mut inner_func);
    }
}

// Recursive function that wraps the function parameter, so generics
// won't work
fn base2_palindrome_loop(n_digits: u8, found_func: &mut dyn FnMut(u32)) {
    if n_digits == 0 {
        found_func(0);
    } else if n_digits == 1 {
        found_func(0);
        found_func(1);
    } else {
        let mut zero_func = |x| found_func(2 * x);
        base2_palindrome_loop(n_digits - 2, &mut zero_func);
        let high_added: u32 = 1 << (n_digits - 1);
        let mut one_func = |x| found_func(high_added + 2 * x + 1);
        base2_palindrome_loop(n_digits - 2, &mut one_func);
    }
}

fn check_base10_palindrome(n: u32) -> bool {
    if n >= 1000000 {
        // only care about up to 1 million
        return false;
    }
    if n < 10 {
        return true;
    }
    let mut pow_10 = 10;
    let mut ndig = 1;
    while pow_10 < n {
        pow_10 *= 10;
        ndig += 1;
    }
    return check_base10_palindrome_2(n, ndig);
}

fn check_base10_palindrome_2(n: u32, ndigits: u8) -> bool {
    if ndigits < 2 {
        return true;
    }
    let mut top_of_n = n;
    let mut pow_10 = 1;
    for _ in 1..ndigits {
        top_of_n /= 10;
        pow_10 *= 10;
    }
    return (top_of_n == n % 10) && (check_base10_palindrome_2((n % pow_10) / 10, ndigits - 2));
}
