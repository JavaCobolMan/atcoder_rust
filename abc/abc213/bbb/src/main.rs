use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut buf: Vec<(usize, &usize)> = a.iter().enumerate().map(|(i, x)| (i + 1, x)).collect();
    buf.sort_by(|x, y| x.1.cmp(y.1));

    println!("{}", buf[n - 2].0);
}
