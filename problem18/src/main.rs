use std::cmp::max;

fn parse_grid(gridstr: &str) -> Vec<Vec<u16>> {
    gridstr
        .trim()
        .split("\n")
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|numstr| numstr.parse().expect("Expected a u8"))
                .collect()
        })
        .collect()
}

fn main() {
    let gridstr = r"
75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23
";
    let mut grid: Vec<Vec<u16>> = parse_grid(gridstr);
    let mut working: Vec<u16> = grid.pop().unwrap();
    while working.len() > 1 {
        let max_to_add = working[1..]
            .iter()
            .zip(&working[..working.len() - 1])
            .map(|(a, b)| max(a, b));
        working = grid
            .pop()
            .unwrap()
            .iter()
            .zip(max_to_add)
            .map(|(a, b)| a + b)
            .collect();
    }
    println!("{}", working[0]);
}
