use std::convert::TryFrom;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::str;

fn main() {
    let path = Path::new("names.txt");
    let nameholder: Vec<Vec<u8>> = File::open(&path)
        .map(BufReader::new)
        .unwrap()
        .split(',' as u8)
        .map(|x| x.unwrap())
        .collect();

    let mut names: Vec<&str> = nameholder.iter().map(|x| find_name(&x[..])).collect();
    names.sort_unstable();
    let score_sum: u32 = names
        .iter()
        .enumerate()
        .map(|(idx, name)| (u32::try_from(idx + 1).unwrap()) * name_score(name))
        .sum();
    println!("{}", score_sum);
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
