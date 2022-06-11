use proconio::marker::*;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [Usize1; k],
        x: [(isize, isize); n],
    }

    let mut buf: Vec<isize> = vec![];
    for i in 0..n {
        if a.contains(&i) {
            continue;
        } else {
            let mut tmp: Vec<isize> = vec![];
            for j in 0..n {
                if a.contains(&j) {
                    tmp.push(
                        (x[i].0 - x[j].0) * (x[i].0 - x[j].0)
                            + (x[i].1 - x[j].1) * (x[i].1 - x[j].1),
                    );
                } else {
                    continue;
                }
            }
            tmp.sort();
            buf.push(tmp[0]);
        }
    }
    buf.sort();
    let d = buf.pop().unwrap();

    println!("{}", (d as f64).sqrt());
}
