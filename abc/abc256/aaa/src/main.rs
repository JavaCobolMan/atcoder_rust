use proconio::*;

#[fastout]
fn main() {
    input! {
        n: u32,
    }

    let two: usize = 2;
    println!("{}", two.pow(n));
}
