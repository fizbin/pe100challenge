const MONTH_DAYS: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn main() {
    let mut month = 1;
    let mut year = 1900;
    let mut wkday = 1;

    assert_eq!(MONTH_DAYS.iter().sum::<i32>(), 365);
    // advance to Jan 1 1901
    wkday = (wkday + 366) % 7;
    year += 1;

    let mut n_sundays = 0;

    while year < 2001 {
        // tally if this month begins on a Sunday, then add a month
        if wkday == 0 {
            n_sundays += 1;
        }
        let mut mdays = MONTH_DAYS[month - 1];
        if year % 4 == 0 && month == 2 {
            mdays += 1;
        }
        wkday = (wkday + mdays) % 7;
        month = (month % 12) + 1;
        if month == 1 {
            year += 1;
        }
    }
    println!("{}", n_sundays);
}
