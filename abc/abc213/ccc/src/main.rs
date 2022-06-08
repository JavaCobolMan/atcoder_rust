use proconio::*;
use std::collections::BTreeMap;

#[fastout]
fn main() {
    input! {
        _h: usize,
        _w: usize,
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut map_h: BTreeMap<usize, usize> = BTreeMap::new();
    let mut map_w: BTreeMap<usize, usize> = BTreeMap::new();
    for i in 0..n {
        map_h.insert(ab[i].0, 0);
        map_w.insert(ab[i].1, 0);
    }

    let mut cnt: usize = 0;
    for (k, v) in map_h.iter_mut() {
        cnt += 1;
        *v = k - cnt;
    }

    let mut cnt: usize = 0;
    for (k, v) in map_w.iter_mut() {
        cnt += 1;
        *v = k - cnt;
    }

    for i in 0..n {
        println!(
            "{} {}",
            ab[i].0 - map_h.get(&ab[i].0).unwrap(),
            ab[i].1 - map_w.get(&ab[i].1).unwrap(),
        );
    }
}
