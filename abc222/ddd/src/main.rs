use proconio::input;
use std::collections::HashMap;

const D: usize = 998244353;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
        bn: [usize; n],
    }
    let mut end_cnt_map: HashMap<usize, usize> = HashMap::new();
    let mut tmp_cnt1 = 0;
    for i in an[0]..=bn[0] {
        tmp_cnt1 += 1;
        end_cnt_map.insert(i, tmp_cnt1);
    }
    let mut tmp_cnt1;
    let mut tmp_cnt2;
    for i in 1..n {
        tmp_cnt1 = 0;
        tmp_cnt2 = *end_cnt_map.get(&bn[i - 1]).unwrap();
        for j in an[i]..=bn[i] {
            if bn[i - 1] >= j {
                tmp_cnt2 = *end_cnt_map.get(&j).unwrap();
                tmp_cnt1 += tmp_cnt2;
            } else {
                tmp_cnt1 += tmp_cnt2;
            }
            tmp_cnt1 %= D;
            end_cnt_map.insert(j, tmp_cnt1);
        }
    }
    println!("{}", end_cnt_map.get(&bn[n - 1]).unwrap());
}
