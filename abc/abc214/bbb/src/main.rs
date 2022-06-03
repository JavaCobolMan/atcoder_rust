use proconio::*;

#[fastout]
fn main() {
    input! {
        s: usize,
        t: usize,
    }

    let mut v: Vec<[usize; 3]> = vec![];
    for i in 0..=s {
        for j in 0..=s - i {
            for k in 0..=s - i - j {
                v.push([i, j, k]);
            }
        }
    }

    let mut cnt = 0;
    for e in v {
        if e[0] * e[1] * e[2] <= t {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
