fn main() {
    let mut total = 0;
    iterate_over_pandigitals(&mut |val, digits: &[u64]| {
        if ((digits[1] * 100 + digits[2] * 10 + digits[3]) % 2 == 0)
            && ((digits[2] * 100 + digits[3] * 10 + digits[4]) % 3 == 0)
            && ((digits[3] * 100 + digits[4] * 10 + digits[5]) % 5 == 0)
            && ((digits[4] * 100 + digits[5] * 10 + digits[6]) % 7 == 0)
            && ((digits[5] * 100 + digits[6] * 10 + digits[7]) % 11 == 0)
            && ((digits[6] * 100 + digits[7] * 10 + digits[8]) % 13 == 0)
            && ((digits[7] * 100 + digits[8] * 10 + digits[9]) % 17 == 0)
        {
            println!("found {}", val);
            total += val;
        }
    });
    println!("{}", total);
}

fn iterate_over_pandigitals(f: &mut dyn FnMut(u64, &[u64])) {
    // Iterates from the largest pandigital number of length n on down

    let n_usize = usize::from(10u8);
    let n_u64 = u64::from(10u8);
    let mut digits = vec![10; n_usize];

    let mut dig_index: usize = 1;

    while dig_index > 0 {
        if digits[dig_index - 1] == 0 {
            digits[dig_index - 1] = n_u64;
            dig_index -= 1;
            continue;
        }
        digits[dig_index - 1] -= 1;
        // Turns out that this constraint cuts the time by more than a third
        if dig_index == 6 && digits[dig_index - 1] != 5 && digits[dig_index - 1] != 0 {
            continue;
        }
        if dig_index == 5 && ((digits[2] + digits[3] + digits[4]) % 3 != 0) {
            continue;
        }

        if digits[0..(dig_index - 1)].contains(&digits[dig_index - 1]) {
            continue;
        }

        if dig_index == n_usize {
            let val: u64 = digits.iter().fold(0, |acc, i| 10 * acc + *i);
            &f(val, &digits);
        } else {
            dig_index += 1;
        }
    }
}
