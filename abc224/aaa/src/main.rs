use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let s_len = s.len();
    let er = String::from("er");
    let ist = String::from("ist");

    if &s[s_len - 2..] == &er {
        println!("{}", &er);
    } else {
        println!("{}", &ist);
    }
}
