use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::str;

fn main() {
    let path = Path::new("words.txt");
    let ans = File::open(&path)
        .map(BufReader::new)
        .unwrap()
        .split(',' as u8)
        .map(|x| name_score(find_name(&x.unwrap())))
        .map(|m| 8 * m + 1)
        .fold(0, |acc, i| if is_perfect_square(i) { acc + 1 } else { acc });
    println!("{}", ans);
}

fn name_score(name: &str) -> u32 {
    name.chars()
        .map(|c| match c {
            'A'..='Z' => (c as u32) - 64, // Yeah, hard coded because...
            _ => panic!("Bad name {}", name),
        })
        .sum()
}

fn find_name(underlying: &[u8]) -> &str {
    let mut first_letter: usize = 0;
    let mut last_letter: usize;

    while first_letter < underlying.len() && !(underlying[first_letter] as char).is_alphabetic() {
        first_letter += 1;
    }

    last_letter = first_letter;

    while last_letter < underlying.len() && (underlying[last_letter] as char).is_alphabetic() {
        last_letter += 1;
    }

    str::from_utf8(&underlying[first_letter..last_letter]).unwrap()
}

fn is_perfect_square(n: u32) -> bool {
    let mut tst = 1;
    let mut tmp = n;
    while tmp > 1 {
        tst <<= 1;
        tmp >>= 2;
    }
    while tst != tmp && tst > 0 {
        tmp = tst;
        tst = ((n / tst) + tst) / 2;
    }
    tst * tst == n
}
