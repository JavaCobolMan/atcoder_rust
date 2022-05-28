use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        xyn: [(usize, usize); n],
    }

    let xyn: Vec<(usize, usize)> = xyn;
    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    for (x, y) in xyn {
        let o = map.get_mut(&x);
        match o {
            Some(v) => {
                v.push(y);
            }
            None => {
                map.insert(x, vec![y]);
            }
        };
    }

    let mut tmp: Vec<Vec<usize>> = vec![];
    for (_, val) in map {
        if val.len() > 1 {
            tmp.push(val);
        }
    }

    let mut cnt = 0;
    if tmp.len() < 2 {
        println!("{}", cnt);
        return;
    }

    for i in 0..tmp.len() - 1 {
        for j in i + 1..tmp.len() {
            for k in 0..tmp[i].len() - 1 {
                let v1 = tmp[i][k];
                if tmp[j].contains(&v1) {
                    for l in k + 1..tmp[i].len() {
                        let v2 = tmp[i][l];
                        if tmp[j].contains(&v2) {
                            cnt += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", cnt);
}
