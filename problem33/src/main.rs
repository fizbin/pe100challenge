use num_rational::Ratio;
use num_rational::Rational;

fn main() {
    let mut prod: Rational = Ratio::from(1);
    for a in 1..=9 {
        for d in 1..=9 {
            if d <= a { // we need a/d <= 1
                continue
            }
            for b in 1..=9 {
                for c in 1..=9 {
                    if b == c && Ratio::new(10 * a + b, 10 * c + d) == Ratio::new(a, d) {
                        prod *= Ratio::new(a, d);
                        // println!("{}{} / {}{}", a, b, c, d);
                    }
                }
            }
        }
    }

    println!("{}", prod.denom());
}
