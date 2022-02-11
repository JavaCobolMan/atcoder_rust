use proconio::input;

fn main() {
    input! {
        n: usize,
        (x, y): (usize, usize),
        abn: [(usize, usize); n],
    }
    let max = 10000;
    let tmp_b: Vec<isize> = vec![max; y + 1];
    let tmp_ab: Vec<Vec<isize>> = vec![tmp_b; x + 1];
    let mut tmp_iab: Vec<Vec<Vec<isize>>> = vec![tmp_ab; n + 1];
    tmp_iab[0][0][0] = 0;
    for i in 1..=n {
        let (ai, bi) = abn[i - 1];
        for a in 0..=x {
            for b in 0..=y {
                if tmp_iab[i - 1][a][b] < max {
                    tmp_iab[i][a][b] = std::cmp::min(tmp_iab[i][a][b], tmp_iab[i - 1][a][b]);
                    let tmp_a = std::cmp::min(a + ai, x);
                    let tmp_b = std::cmp::min(b + bi, y);
                    tmp_iab[i][tmp_a][tmp_b] =
                        std::cmp::min(tmp_iab[i - 1][a][b] + 1, tmp_iab[i][tmp_a][tmp_b]);
                }
            }
        }
    }
    println!(
        "{}",
        if tmp_iab[n][x][y] < max {
            tmp_iab[n][x][y]
        } else {
            -1
        }
    )
}
