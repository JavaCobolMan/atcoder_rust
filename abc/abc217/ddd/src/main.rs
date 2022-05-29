use proconio::*;
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input! {
        l: usize,
        q: usize,
        cxq: [(usize, usize); q],
    }
    let mut tmp1 = BTreeSet::<usize>::new();
    tmp1.insert(0);
    tmp1.insert(l);

    for (c, x) in cxq {
        if c == 1 {
            tmp1.insert(x);
        }
        if c == 2 {
            let r2 = tmp1.range(..x).last().unwrap();
            let r1 = tmp1.range(x..).nth(0).unwrap();
            println!("{}", r1 - r2);
        }
    }
}
