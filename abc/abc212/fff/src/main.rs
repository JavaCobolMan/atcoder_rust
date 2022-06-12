use proconio::*;

//#[fastout]
fn main() {
    input! {
        _n: usize,
        m: usize,
        q: usize,
        mut abst: [(usize, usize, usize, usize); m],
        xyz: [(usize, usize, usize); q],
    }

    abst.sort_by(|a, b| (a.2).cmp(&(b.2)));
    let mut dp: Vec<Vec<usize>> = vec![vec![0; m]; 22];
    for i in 0..m {
        dp[0][i] = i;
    }
    for i in 0..m {
        let mut tmp = i;
        for j in 0..m {
            if abst[i].1 == abst[j].0 && abst[i].3 <= abst[j].2 {
                tmp = j;
                break;
            }
        }
        dp[1][i] = tmp;
    }

    for i in 2..dp.len() {
        for j in 0..m {
            dp[i][j] = dp[i - 1][dp[i - 1][j]];
        }
    }

    for (x, y, z) in xyz {
        let mut tmp: Option<usize> = None;
        for i in 0..m {
            if y == abst[i].0 && x <= abst[i].2 {
                if z > abst[i].2 {
                    tmp = Some(i);
                }
                break;
            }
        }

        match tmp {
            None => {
                println!("{}", y);
            }
            Some(x) => {
                let mut i = x;
                'a: loop {
                    let mut buf: isize = -1;
                    for j in 0..dp.len() {
                        let tmp = dp[j][i];
                        if z <= abst[tmp].2 {
                            if j == 0 || j == 1 {
                                println!("{}", abst[tmp].0);
                                break 'a;
                            } else {
                                i = dp[j - 1][i];
                                break;
                            }
                        } else if z <= abst[tmp].3 {
                            println!("{} {}", abst[tmp].0, abst[tmp].1);
                            break 'a;
                        } else {
                            if buf == tmp as isize {
                                println!("{}", abst[tmp].1);
                                break 'a;
                            } else {
                                buf = tmp as isize;
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }
}
