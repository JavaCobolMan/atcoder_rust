use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut q: Vec<String> = vec![String::from(""); n];
    for (i, v) in p.iter().enumerate() {
        q[*v - 1] = (i + 1).to_string();
    }

    println!("{}", q.join(" "));
}
