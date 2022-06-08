use proconio::*;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }

    println!("{}", a ^ b);
}
