use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
    }
    let mut i = 1;
    let r: isize = loop {
        if a <= c * i {
            if b >= c * i {
                break c * i;
            } else {
                break -1;
            }
        }
        i += 1;
    };

    println!("{}", r);
}
