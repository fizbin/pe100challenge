use core::cmp::Ordering;
use rug::Integer;
use std::convert::TryFrom;

fn main() {
    let mut longpow = Integer::from(Integer::factorial(100));
    let mut digsum = 0u16;
    while longpow.cmp0() == Ordering::Greater {
        let mut digit = Integer::from(10);
        longpow.div_rem_mut(&mut digit);
        digsum += u16::try_from(digit).unwrap();
    }
    
    println!("{}", digsum);
}
