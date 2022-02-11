use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        mut x: String,
        n: usize,
        sn: [String; n],
    }
    let mut x_map: BTreeMap<char, usize> = BTreeMap::new();
    let mut i: usize = 10;
    while x.len() != 0 {
        i += 1;
        x_map.insert(x.remove(0), i);
    }

    let mut tmp_map: BTreeMap<String, String> = BTreeMap::new();
    for s in sn {
        let mut tmp1: String = String::new();
        for c in s.chars() {
            tmp1.push_str(&x_map.get(&c).unwrap().to_string());
        }
        tmp_map.insert(tmp1, s);
    }

    for (_, v) in tmp_map.iter() {
        println!("{}", v);
    }
}
