use proconio::*;

#[fastout]
fn main() {
    input! {
        h1: usize,
        h2: usize,
        h3: usize,
        w1: usize,
        w2: usize,
        w3: usize,
    }

    if h1 + h2 + h3 == w1 + w2 + w3 {
        let h1_vec = get_vec(h1);
        let h2_vec = get_vec(h2);
        let h3_vec = get_vec(h3);

        let mut cnt = 0;
        for v1 in &h1_vec {
            if v1.0 <= w1 - 2 && v1.1 <= w2 - 2 && v1.2 <= w3 - 2 {
                for v2 in &h2_vec {
                    if v1.0 + v2.0 <= w1 - 1 && v1.1 + v2.1 <= w2 - 1 && v1.2 + v2.2 <= w3 - 1 {
                        for v3 in &h3_vec {
                            if v1.0 + v2.0 + v3.0 == w1
                                && v1.1 + v2.1 + v3.1 == w2
                                && v1.2 + v2.2 + v3.2 == w3
                            {
                                cnt += 1;
                                break;
                            }
                        }
                    }
                }
            }
        }

        println!("{}", cnt);
    } else {
        println!("{}", 0);
    }
}

fn get_vec(n: usize) -> Vec<(usize, usize, usize)> {
    let mut buf: Vec<(usize, usize, usize)> = vec![];
    for i in 1..n - 1 {
        for j in 1..n - i {
            let tmp = (i, j, n - i - j);
            buf.push(tmp);
        }
    }
    return buf;
}
