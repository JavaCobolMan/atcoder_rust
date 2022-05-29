use proconio::*;

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let s: Vec<&str> = s.split(".").collect();
    let mut x = String::from(s[0]);
    let y: usize = s[1].parse().unwrap();
    if y <= 2 {
        x.push_str("-");
    } else if 7 <= y {
        x.push_str("+");
    }
    println!("{}", x);
}
