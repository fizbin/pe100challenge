fn main() {
    let mut ans = 1;
    let mut n = 1;
    let mut goal_d = 1;
    let mut sofar = 0;
    while goal_d < 10000000 {
        let dign = n.to_string();
        let remaining = goal_d - sofar;
        if remaining <= dign.len() {
            let target_d = &dign[(remaining-1)..=(remaining-1)];
            ans *= target_d.parse::<u64>().unwrap();
            goal_d *= 10;
        }
        sofar += dign.len();
        n += 1;
    }
    println!("{}", ans);
}
