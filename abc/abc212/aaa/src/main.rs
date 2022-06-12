use proconio::*;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let res;
    if a == 0 {
        res = "Silver";
    } else if b == 0 {
        res = "Gold";
    } else {
        res = "Alloy";
    }

    println!("{}", res);
}
