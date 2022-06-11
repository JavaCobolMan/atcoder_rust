use proconio::marker::*;
use proconio::*;

#[fastout]
fn main() {
    input! {
        r: Usize1,
        c: Usize1,
        a: [[usize; 2]; 2],
    }
    println!("{}", a[r][c]);
}
