use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        abn: [(usize, usize); n],
    }
    let mut tmp_map: BTreeMap<usize, isize> = BTreeMap::new();
    for &(a, b) in &abn {
        let tmp: isize;
        match tmp_map.get(&a) {
            Option::None => tmp = 1,
            Option::Some(x) => tmp = x + 1,
        }
        tmp_map.insert(a, tmp);
        let tmp: isize;
        match tmp_map.get(&(a + b)) {
            Option::None => tmp = -1,
            Option::Some(x) => tmp = x - 1,
        }
        tmp_map.insert(a + b, tmp);
    }

    let mut r: Vec<usize> = vec![0; n + 1];
    let mut cnt: isize = 0;
    let mut cnt_index = 0;
    for (k, v) in tmp_map.iter() {
        if *v == 0 {
            continue;
        }
        if cnt != 0 {
            r[cnt as usize] += *k - cnt_index;
        }
        cnt += *v;
        cnt_index = *k;
    }
    r.remove(0);
    let r: Vec<String> = r.iter().map(|x| x.to_string()).collect();
    println!("{}", r.join(" "));
}
