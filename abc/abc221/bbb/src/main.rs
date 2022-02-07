use proconio::input;

fn main() {
    input! {
        mut s: String,
        mut t: String,
    }
    let mut r = "Yes";
    while s.len() != 0 {
        let tmp_s = s.pop().unwrap();
        let tmp_t = t.pop().unwrap();
        if tmp_s != tmp_t {
            let buff_s = s.pop().unwrap();
            let buff_t = t.pop().unwrap();
            if tmp_s == buff_t && tmp_t == buff_s {
                if s == t {
                    break;
                }
            }
            r = "No";
            break;
        }
    }
    println!("{}", r);
}
