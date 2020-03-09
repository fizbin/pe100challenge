fn main() {
    let mut d_record: Vec<u32> = Vec::with_capacity(10001);
    d_record.push(0);
    for x in 1..10000 {
        d_record.push(divisor_sum(x));
    }
    let mut sum = 0;
    for x in 1..10000 {
        let dx = d_record[x as usize];
        if dx != x && (dx as usize) < d_record.len() && x == d_record[dx as usize] {
            sum += x;
        }
    }
    println!("{}", sum);
}

fn divisor_sum(n: u32) -> u32 {
    let mut sum = 1;
    let mut divisor = 2;
    while divisor * divisor < n {
        if n % divisor == 0 {
            sum += divisor;
            sum += n / divisor;
        }
        divisor += 1;
    }
    if divisor * divisor == n {
        sum += divisor;
    }
    sum
}
