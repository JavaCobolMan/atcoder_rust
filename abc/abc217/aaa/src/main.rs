use proconio::input;
use proconio::marker::Chars;

use std::cmp;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut r = "";
    for i in 0..cmp::max(s.len(), t.len()) {
        if get_ele(&s, i) < get_ele(&t, i) {
            r = "Yes";
            break;
        } else if get_ele(&s, i) > get_ele(&t, i) {
            r = "No";
            break;
        } else {
            continue;
        }
    }

    println!("{}", r);
}

fn get_ele(st: &Vec<char>, i: usize) -> usize {
    if i >= st.len() {
        return 0;
    } else {
        return st[i] as usize;
    }
}
