use proconio::*;
use std::collections::BTreeMap;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        _n: usize,
        m: usize,
        mut a: [[usize]; m],
    }

    let mut map: BTreeMap<usize, usize> = BTreeMap::new();
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();

    for i in 0..m {
        f(&mut a, &mut map, &mut q, i);
    }

    loop {
        let v = q.pop_front();
        match v {
            None => {
                break;
            }
            Some((i1, i2)) => {
                f(&mut a, &mut map, &mut q, i1);
                f(&mut a, &mut map, &mut q, i2);
            }
        }
    }

    println!(
        "{}",
        if a.iter().map(|x| x.len()).sum::<usize>() == 0 {
            "Yes"
        } else {
            "No"
        }
    );
}

fn f(
    a: &mut Vec<Vec<usize>>,
    map: &mut BTreeMap<usize, usize>,
    q: &mut VecDeque<(usize, usize)>,
    i: usize,
) {
    if let Some(v) = a[i].pop() {
        if map.contains_key(&v) {
            let tmp_i = map.remove(&v).unwrap();
            q.push_back((i, tmp_i));
        } else {
            map.insert(v, i);
        }
    }
}
