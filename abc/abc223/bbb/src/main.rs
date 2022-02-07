use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut s_min = s.clone();
    let mut s_max = s.clone();
    for i in 1..=s.len() {
        let s_buf: String = String::from(&s[i..]) + &s[..i];
        if s_min > s_buf {
            s_min = s_buf.clone();
        }
        if s_max < s_buf {
            s_max = s_buf;
        }
    }
    println!("{}", s_min);
    println!("{}", s_max);
}
