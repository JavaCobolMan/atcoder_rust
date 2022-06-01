use proconio::marker::*;
use proconio::*;
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input! {
        s: Chars,
        k: Usize1,
    }

    let mut set: BTreeSet<String> = BTreeSet::new();
    put_set(&mut set, s, String::from(""));
    let res = set.iter().skip(k).next().unwrap();
    println!("{}", res);
}

fn put_set(set: &mut BTreeSet<String>, c: Vec<char>, s: String) {
    if c.len() == 0 {
        set.insert(s);
    } else {
        for i in 0..c.len() {
            let mut c_tmp = c.clone();
            let mut s_tmp = s.clone();
            s_tmp.push(c_tmp.remove(i));
            put_set(set, c_tmp, s_tmp);
        }
    }
}
