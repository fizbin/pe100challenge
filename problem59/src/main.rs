use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::Read;
// use std::io::Write;
use std::iter::FromIterator;
use std::path::Path;

fn read_csv_to_u8(filename: &Path) -> Result<Vec<u8>, io::Error> {
    let file = File::open(filename)?;
    let mut reader = io::BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    content
        .trim()
        .split(',')
        .map(|s| {
            s.trim().parse::<u8>().map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidData, format!("Invalid u8: '{}'", s))
            })
        })
        .collect()
}

fn main() {
    let english_frequencies: HashMap<u8, u32> = HashMap::from_iter([
        (b'e', 12702),
        (b't', 9056),
        (b'a', 8167),
        (b'o', 7507),
        (b'i', 6966),
        (b'n', 6749),
        (b's', 6327),
        (b'h', 6094),
        (b'r', 5987),
        (b'd', 4253),
    ]);
    let bytes;
    match read_csv_to_u8(Path::new("0059_cipher.txt")) {
        Ok(n) => bytes = n,
        Err(err) => panic!("{}", err),
    }
    let mut array_of_freqs: [HashMap<u8, u32>; 3] = [(); 3].map(|_| HashMap::new());
    for (idx, byte) in bytes.iter().enumerate() {
        let vref = array_of_freqs[idx % 3].entry(*byte).or_default();
        *vref += 1;
    }
    let mut key_chars: [u8; 3] = [0, 0, 0];
    for key_pos in 0..3 {
        let mut best_char = b'@';
        let mut best_score: f32 = 99999.0;
        for key_char in b'a'..=b'z' {
            let xformed_freq: HashMap<u8, u32> = HashMap::from_iter(
                array_of_freqs[key_pos]
                    .iter()
                    .map(|(k, v)| (key_char ^ k, *v)),
            );
            let score = l2_distance(&xformed_freq, &english_frequencies);
            if score < best_score {
                best_score = score;
                best_char = key_char;
                // println!("{} -> {}: {}", key_pos, key_char, score);
            }
        }
        key_chars[key_pos] = best_char;
    }
    let mut total = 0 as u32;
    for (key_pos, key_char) in key_chars.iter().enumerate() {
        total += array_of_freqs[key_pos]
            .iter()
            .map(|(k, v)| v * ((k ^ key_char) as u32))
            .sum::<u32>();
    }
    // let mut w = std::io::stdout();
    // let _ = bytes
    //     .iter()
    //     .enumerate()
    //     .map(|(idx, byte)| w.write(&[byte ^ key_chars[idx % 3]]))
    //     .collect::<Vec<_>>();
    println!();
    println!();
    println!("{}", total);
}

fn normalize_freqmap(input: &HashMap<u8, u32>) -> HashMap<u8, f32> {
    // Returns a map only of the letters 'e', 't', 'a', 'o', 'i', 'n', 's', 'h', 'r', 'd'
    let mut retval = HashMap::new();
    let common_letters =
        HashSet::<u8>::from_iter([b'e', b't', b'a', b'o', b'i', b'n', b's', b'h', b'r', b'd']);
    let mut total = 0u32;
    for (lref, count) in input.iter() {
        let mut letter = *lref;
        if letter.is_ascii_uppercase() {
            letter += b'a' - b'A';
        }
        if common_letters.contains(&letter) {
            total += count;
            let valref = retval.entry(letter).or_insert(0.0);
            *valref = *valref + (*count as f32);
        }
    }
    let totalf = total as f32;
    for (_, count) in retval.iter_mut() {
        *count /= totalf;
    }
    retval
}

fn l2_distance(input1: &HashMap<u8, u32>, input2: &HashMap<u8, u32>) -> f32 {
    let map1 = normalize_freqmap(input1);
    let map2 = normalize_freqmap(input2);
    let mut l2dist: f32 = 0.0;
    for key in map1
        .keys()
        .copied()
        .collect::<HashSet<_>>()
        .union(&map2.keys().copied().collect())
    {
        let sub = map1.get(key).unwrap_or(&0.0) - map2.get(key).unwrap_or(&0.0);
        l2dist += sub * sub;
    }
    return l2dist.sqrt();
}
