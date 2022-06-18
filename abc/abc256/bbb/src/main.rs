use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    for i in 0..n {
        let tmp = a[i];
        for j in 0..i {
            a[j] += tmp;
        }
    }

    let mut cnt: usize = 0;
    for v in a {
        if v > 3 {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
