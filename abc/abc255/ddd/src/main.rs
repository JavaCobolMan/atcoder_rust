use proconio::*;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        x: [usize; q],
    }

    a.sort();
    let mut x_tmp = x.clone();
    x_tmp.sort();
    let mut map: HashMap<usize, usize> = HashMap::new();

    let mut i = 0;
    let mut n_sum = 0;
    let mut n_cnt = 0;
    let mut p_sum: usize = a.iter().sum();
    let mut p_cnt = a.len();
    for v in x_tmp {
        for j in i..n {
            if a[j] < v {
                i += 1;
                n_sum += a[j];
                n_cnt += 1;
                p_sum -= a[j];
                p_cnt -= 1;
            } else {
                break;
            }
        }

        let res = (v * n_cnt - n_sum) + (p_sum - v * p_cnt);
        map.insert(v, res);
    }

    for v in x {
        println!("{}", map.get(&v).unwrap());
    }
}
