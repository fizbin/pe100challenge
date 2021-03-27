# Problem 48: Self powers

## Problem statement

<p>The series, 1<sup>1</sup> + 2<sup>2</sup> + 3<sup>3</sup> + ... + 10<sup>10</sup> = 10405071317.</p>
<p>Find the last ten digits of the series, 1<sup>1</sup> + 2<sup>2</sup> + 3<sup>3</sup> + ... + 1000<sup>1000</sup>.</p>

## Comments

The key here is basically to keep only the last ten digits of any of the arithmetic - the powers, addition, etc.

...

Two problems: one stupid self-imposed screw-up in that I used `pow` when I meant `base` in one of my
recursive cases for `pow10dig`, and the other in that because of how I structured `pow10dig`: I wrote
it so that computing <i>a<sup>b</sup></i> would take O(log <i>b</i>) multiplications, but unfortunately
that results in a u64 overflow at some point because 9999999999 * 9999999999 is indeed larger than can
be represented in a u64. (u64 max is only 19 digits long, so it isn't surprising that two 10-digit
numbers multiplied together would overflow)

To fix the second problem, I wrote my own non-overflowing multiply-two-numbers-and-get-the-last-ten-digits routine:

    fn mul10dig(a: u64, b: u64) -> u64 {
        let alow = a % FIVE_ZEROS;
        let blow = b % FIVE_ZEROS;
        let ahigh = a - alow;
        let bhigh = b - blow;
        return (alow * blow + ahigh * blow + alow * bhigh) % TEN_ZEROS;
    }
