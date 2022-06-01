use proconio::*;

const HW: &str = "Hello,World!";

#[fastout]
fn main() {
    input! {
        s: String,
    }
    println!("{}", if s == HW { "AC" } else { "WA" });
}
