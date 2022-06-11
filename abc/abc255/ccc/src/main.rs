use proconio::*;

#[fastout]
fn main() {
    input! {
        x: isize,
        a: isize,
        d: isize,
        n: isize,
    }

    let tmp1 = a;
    let tmp2 = a + d * (n - 1);
    if x <= tmp1.min(tmp2) {
        println!("{}", tmp1.min(tmp2) - x);
    } else if x > tmp1.max(tmp2) {
        println!("{}", x - tmp1.max(tmp2));
    } else {
        let tmp = (x - a) / d;
        let tmp1 = (x - (a + d * (tmp))).abs();
        let tmp2 = (x - (a + d * (tmp + 1))).abs();
        println!("{}", tmp1.min(tmp2));
    }
}
