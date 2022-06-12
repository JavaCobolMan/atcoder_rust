use proconio::*;
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut query: Vec<(usize, isize)> = vec![(0, 0); q];
    for i in 0..q {
        input! {
            p: usize,
        }
        query[i].0 = p;
        if p != 3 {
            input! {
                x: isize,
            }
            query[i].1 = x;
        }
    }

    let mut buf1: BinaryHeap<isize> = BinaryHeap::new();
    let mut buf2 = 0;
    for v in query {
        if v.0 == 1 {
            buf1.push(-(v.1 - buf2));
        } else if v.0 == 2 {
            buf2 += v.1;
        } else {
            println!("{}", -buf1.pop().unwrap() + buf2);
        }
    }
}
