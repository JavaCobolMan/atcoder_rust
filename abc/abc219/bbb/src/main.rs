use proconio::input;

fn main() {
    input! {
        s1: String,
        s2: String,
        s3: String,
        mut t: String,
    }
    let s123 = [s1, s2, s3];
    let mut r: String = String::new();
    while t.len() != 0 {
        r.push_str(&s123[t.remove(0) as usize - 49]);
    }
    println!("{}", r);
}
