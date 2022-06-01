use proconio::*;

#[fastout]
fn main() {
    input! {
        n: u64,
    }

    let mut k: u32 = 0;
    while 2u64.pow(k) <= n {
        k += 1;
    }

    println!("{}", k - 1);
}
