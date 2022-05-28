use proconio::input;

fn main() {
    input! {
        p: [u8; 26],
    }

    let mut s: Vec<char> = Vec::new();
    for val in p.iter() {
        s.push((val + 96) as char);
    }
    let s: String = s.iter().collect();
    println!("{}", s);
}
