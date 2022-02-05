use proconio::input;

fn main() {
    input! {
        n: String,
    }

    let mut n = n;
    while n.len() < 4 {
        n.insert_str(0, "0");
    }
    println!("{}", n);
}
