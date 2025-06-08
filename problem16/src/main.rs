use dashu::base::DivRemAssign;
use dashu::{ibig, integer::IBig};

fn main() {
    let mut longpow = ibig!(2).pow(1000);
    let mut digsum = 0u16;
    while longpow > IBig::ZERO {
        let digit = longpow.div_rem_assign(10 as u16);
        digsum += digit;
    }

    println!("{}", digsum);
}
