use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: Usize1,
        s: Chars,
    }

    println!("{}", if s[n] == 'o' { "Yes" } else { "No" });
}
