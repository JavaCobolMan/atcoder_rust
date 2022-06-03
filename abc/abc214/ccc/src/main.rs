use proconio::*;
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [usize; n],
        t: [usize; n],
    }

    let mut res: Vec<usize> = vec![0; n];
    res[0] = t[0];
    for i in 0..n - 1 {
        res[i + 1] = min(res[i] + s[i], t[i + 1]);
    }

    if res[0] > res[n - 1] + s[n - 1] {
        res[0] = res[n - 1] + s[n - 1];
        for i in 0..n - 1 {
            res[i + 1] = min(res[i] + s[i], t[i + 1]);
        }
    }

    for v in res {
        println!("{}", v);
    }
}
