use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut r = 0;
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            for k in j + 1..n {
                if (xy[j].0 - xy[i].0) * (xy[k].1 - xy[i].1)
                    - (xy[k].0 - xy[i].0) * (xy[j].1 - xy[i].1)
                    != 0
                {
                    r += 1;
                }
            }
        }
    }

    println!("{}", r);
}
