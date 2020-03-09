fn main() {
    let mut lettersum = 0;
    for num in 1 .. 1001 {
        lettersum += to_english(num).chars().filter(|c| c.is_alphabetic()).count();
    }
    println!("{}", lettersum);
}

fn to_english(num: u16) -> String {
    if num < 20 {
        let small_nums: Vec<_> = r"zero one two three four five six seven
                eight nine ten eleven twelve thirteen fourteen fifteen sixteen
                seventeen eighteen nineteen"
            .split_whitespace()
            .collect();
        return String::from(small_nums[num as usize]);
    }
    if num < 100 && num % 10 == 0 {
        let tens: Vec<&str> = "twenty thirty forty fifty sixty seventy eighty ninety"
            .split_whitespace()
            .collect();
        return String::from(tens[(num / 10 - 2) as usize]);
    }
    if num < 100 {
        return format!("{}-{}", to_english((num / 10) * 10), to_english(num % 10));
    }
    if num < 1000 {
        let hundreds = num / 100;
        if num == 100 * hundreds {
            return format!("{} hundred", to_english(hundreds));
        }
        return format!(
            "{} hundred and {}",
            to_english(hundreds),
            to_english(num - 100 * hundreds)
        );
    }
    let thousands = num / 1000;
    if num == 1000 * thousands {
        return format!("{} thousand", to_english(thousands));
    }
    return format!(
        "{} thousand, {}",
        to_english(thousands),
        to_english(num - 1000 * thousands)
    );
}
