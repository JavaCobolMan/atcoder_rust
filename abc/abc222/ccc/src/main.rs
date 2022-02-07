use proconio::input;
use std::collections::BTreeMap;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a2nm: [String; n*2],
    }

    let mut w_map: BTreeMap<usize, i32> = BTreeMap::new();
    for i in 0..n * 2 {
        w_map.insert(i, 0);
    }
    // 処理
    for _ in 0..m {
        let ranks = get_rank(&w_map);
        for j in 0..n {
            let rank1 = ranks[j * 2];
            let rank2 = ranks[j * 2 + 1];
            let tmp_win = get_win(a2nm[rank1].remove(0), a2nm[rank2].remove(0));
            if tmp_win == 1 {
                w_map.insert(rank1, w_map.get(&rank1).unwrap() + 1);
            } else if tmp_win == -1 {
                w_map.insert(rank2, w_map.get(&rank2).unwrap() + 1);
            }
        }
    }
    for rank in get_rank(&w_map) {
        println!("{}", rank + 1)
    }
}

fn get_win(a1: char, a2: char) -> i32 {
    let mut tmp_w = 0;
    if a1 == a2 {
    } else if a1 == 'G' && a2 == 'C' {
        tmp_w = 1;
    } else if a1 == 'C' && a2 == 'P' {
        tmp_w = 1;
    } else if a1 == 'P' && a2 == 'G' {
        tmp_w = 1;
    } else {
        tmp_w = -1;
    }
    return tmp_w;
}

fn get_rank(w_map: &BTreeMap<usize, i32>) -> Vec<usize> {
    let mut ranks: Vec<usize> = Vec::new();
    let mut tmp_set: HashSet<usize> = HashSet::new();
    while w_map.len() != tmp_set.len() {
        let mut f = false;
        let mut max = -1;
        let mut tmp_key: usize = 0;
        for (k, v) in w_map.iter() {
            if max < *v && !tmp_set.contains(k) {
                f = true;
                max = *v;
                tmp_key = *k;
            }
        }
        if f {
            tmp_set.insert(tmp_key);
            ranks.push(tmp_key);
        }
    }
    return ranks;
}
