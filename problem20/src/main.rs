use dashu::{ibig, Integer};
use dashu::base::DivRemAssign;


fn main() {
    let mut longpow = ibig!(1);
    for mulfac in 1..=100 {
        longpow *= mulfac;
    }
    let mut digsum = 0u16;
    while longpow > Integer::ZERO {
        let digit = longpow.div_rem_assign(10 as u16);
        digsum += digit;
    }

    println!("{}", digsum);
}
