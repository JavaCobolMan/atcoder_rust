use proconio::marker::*;
use proconio::*;

const W: isize = 998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut path: Vec<Vec<usize>> = vec![vec![]; n];
    for (u, v) in uv {
        path[u].push(v);
        path[v].push(u);
    }

    let mut dp: Vec<Vec<isize>> = vec![vec![0; n]; k + 1];
    dp[0][0] = 1;
    for i in 1..=k {
        let sum: isize = dp[i - 1].iter().sum::<isize>();
        for j in 0..n {
            dp[i][j] = sum - dp[i - 1][j];
            for p in path[j].iter() {
                dp[i][j] -= dp[i - 1][*p];
            }
            dp[i][j] %= W;
        }
    }

    println!("{}", dp[k][0]);
}
