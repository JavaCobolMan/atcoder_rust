use proconio::marker::*;
use proconio::*;

#[fastout]
fn main() {
    input! {
        x: Bytes,
    }

    let mut res = "Strong";

    if x[0] == x[1] && x[1] == x[2] && x[2] == x[3] {
        res = "Weak";
    }

    let mut f = true;
    for i in 0..3 {
        let xi = (x[i] + 1) % 10;
        let xi1 = (x[i + 1]) % 10;
        if xi == xi1 {
            continue;
        } else {
            f = false;
            break;
        }
    }
    if f {
        res = "Weak";
    }

    println!("{}", res);
}
