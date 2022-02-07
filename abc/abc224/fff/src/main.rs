use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut r = 0;
    for i in 0..s.len() {
        r += cl(&s, i);
        r %= 998244353;
    }
    println!("{}", r);
}

fn cl(s: &str, n: usize) -> u128 {
    let mut r: u128 = 0;
    for (i, v) in s.chars().enumerate() {
        let v = v as u128 - 48;
        r += 10u128.pow(i as u32) * v;
    }
    r
}
