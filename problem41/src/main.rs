fn main() {
    // problem statement gives us that a four-digit pandigital prime exists
    // so we can safely call .unwrap()
    println!(
        "{}",
        largest_ndig_pandigital_prime(7)
            .or_else(|| largest_ndig_pandigital_prime(4))
            .unwrap()
    );
}

fn largest_ndig_pandigital_prime(n: u8) -> Option<u64> {
    // Iterates from the largest pandigital number of length n on down

    let n_usize = usize::from(n);
    let n_u64 = u64::from(n);
    let mut digits = vec![n_u64 + 1; n_usize];

    let mut dig_index: usize = 1;

    while dig_index > 0 {
        digits[dig_index - 1] -= 1;
        while digits[0..(dig_index - 1)].contains(&digits[dig_index - 1]) {
            digits[dig_index - 1] -= 1;
        }
        if digits[dig_index - 1] == 0 {
            digits[dig_index - 1] = n_u64 + 1;
            dig_index -= 1;
            continue;
        }
        if dig_index == n_usize {
            let val: u64 = digits.iter().fold(0, |acc, i| 10 * acc + *i);
            if is_prime(val) {
                return Some(val);
            }
        } else {
            dig_index += 1;
        }
    }
    None
}

fn is_prime(testval: u64) -> bool {
    // pulled from problem37
    if testval < 13 {
        match testval {
            2 | 3 | 5 | 7 | 11 => return true,
            _ => return false,
        }
    }
    if (testval % 2 == 0)
        || (testval % 3 == 0)
        || (testval % 5 == 0)
        || (testval % 7 == 0)
        || (testval % 11 == 0)
    {
        return false;
    }
    let mut factor_base = 12;
    while factor_base * factor_base < testval {
        if (testval % (factor_base + 1) == 0)
            || (testval % (factor_base + 5) == 0)
            || (testval % (factor_base + 7) == 0)
            || (testval % (factor_base + 11) == 0)
        {
            return false;
        }
        factor_base += 12;
    }
    true
}
