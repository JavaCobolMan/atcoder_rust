use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    println!(
        "{}",
        if n < 126 {
            4
        } else if n < 212 {
            6
        } else {
            8
        }
    );
}
