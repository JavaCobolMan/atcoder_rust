use proconio::input;

const RANK: [isize; 3] = [40, 70, 90];

fn main() {
    input! {
        x: isize,
    }
    let mut r = -1;
    for &v in &RANK {
        if x < v {
            r = v - x;
            break;
        }
    }

    if r == -1 {
        println!("{}", "expert")
    } else {
        println!("{}", r)
    };
}
