use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        mut an: [usize; n],
    }
    let mut cnt = 0;
    for a in an {
        if a < p {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
