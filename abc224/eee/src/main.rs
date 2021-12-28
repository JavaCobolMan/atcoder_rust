use proconio::marker::*;
use proconio::*;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        p: [(Usize1, Usize1, u32); n],
    }
    let mut p = p
        .into_iter()
        .enumerate()
        .map(|(k, p)| (p.2, p.0 as u32, p.1 as u32, k as u32))
        .collect::<Vec<_>>();
    p.sort_unstable_by_key(|p| !p.0);
    let mut ans = vec![0; n];
    let mut row = vec![0; h];
    let mut col = vec![0; w];
    let mut s = (0, 0);
    for (i, &(a, x, y, k)) in p.iter().enumerate() {
        if s.0 != a {
            for &(_, x, y, k) in p[s.1..i].iter() {
                let x = x as usize;
                let y = y as usize;
                let k = k as usize;
                row[x] = row[x].max(ans[k] + 1);
                col[y] = col[y].max(ans[k] + 1);
            }
            s = (a, i);
        }
        let x = x as usize;
        let y = y as usize;
        let k = k as usize;
        ans[k] = row[x].max(col[y]);
    }
    for a in ans {
        println!("{}", a);
    }
}
