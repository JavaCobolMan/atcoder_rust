use proconio::*;
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }

    let mut set: BTreeSet<usize> = BTreeSet::new();
    for v in a.iter_mut() {
        let mut i: usize = 2;
        while i <= *v {
            if *v % i == 0 {
                *v /= i;
                set.insert(i);
            } else {
                i += 1;
            }
        }
    }

    let mut buf: Vec<usize> = vec![1; m];

    for v in set {
        for i in (v..=m).step_by(v) {
            buf[i - 1] = 0;
        }
    }

    println!("{}", buf.iter().sum::<usize>());
    for (i, v) in buf.iter().enumerate() {
        if *v == 1 {
            println!("{}", i + 1);
        }
    }
}
