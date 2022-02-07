use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let mut r = 1;
    for _ in b..a {
        r *= 32
    }

    println!("{}", r);
}
