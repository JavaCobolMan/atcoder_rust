use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        st: [(String, String); n],
    }

    let mut f = false;
    for i in 0..n - 1 {
        for j in i + 1..n {
            if st[i] == st[j] {
                f = true;
                break;
            }
        }
    }

    println!("{}", if f { "Yes" } else { "No" });
}
