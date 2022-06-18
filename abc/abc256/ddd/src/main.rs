use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut lr: [(usize, usize); n],
    }

    lr.sort_by(|a, b| (a.0).cmp(&b.0));
    let mut buf: Vec<(usize, usize)> = vec![];
    buf.push(lr[0]);

    for i in 0..n {
        let j = buf.len() - 1;
        if lr[i].0 <= buf[j].1 {
            if lr[i].1 > buf[j].1 {
                buf[j].1 = lr[i].1;
            }
        } else {
            buf.push(lr[i]);
        }
    }

    for v in buf {
        println!("{} {}", v.0, v.1);
    }
}
