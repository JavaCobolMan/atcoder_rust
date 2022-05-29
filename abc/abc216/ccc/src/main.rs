use proconio::*;

#[fastout]
fn main() {
    input! {
        mut n: usize,
    }

    let mut s: String = String::from("");
    loop {
        if n == 0 {
            break;
        }

        if n % 2 == 0 {
            n /= 2;
            s.insert_str(0, "B");
        } else {
            n -= 1;
            s.insert_str(0, "A");
        }
    }

    println!("{}", s);
}
