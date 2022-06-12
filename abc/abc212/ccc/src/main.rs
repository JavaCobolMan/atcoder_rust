use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [isize; n],
        mut b: [isize; m],
    }

    a.sort();
    b.sort();

    let mut ai = 0;
    let mut bi = 0;
    let mut dif = (a[ai] - b[bi]).abs();
    loop {
        if ai >= n || bi >= m {
            break;
        }

        let tmp = a[ai] - b[bi];
        if dif > tmp.abs() {
            dif = tmp.abs();
        }
        if tmp < 0 {
            ai += 1;
        } else if tmp > 0 {
            bi += 1;
        } else {
            break;
        }
    }

    println!("{}", dif);
}
