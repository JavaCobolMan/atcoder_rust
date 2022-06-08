use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n-1],
    }

    let mut citys: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for (a, b) in ab {
        citys[a].push(b);
        citys[b].push(a);
    }
    for i in 0..=n {
        citys[i].sort_by(|x, y| y.cmp(x));
    }
    let mut i = 1;
    let mut res: Vec<String> = vec![];
    let mut buf: Vec<usize> = vec![];

    loop {
        let f1 = citys[i].len() == 0;
        if f1 {
            res.push(i.to_string());
            let tmp = buf.pop();
            match tmp {
                None => {
                    break;
                }
                Some(x) => {
                    i = x;
                }
            }
        } else {
            res.push(i.to_string());
            buf.push(i);
            let j = i.clone();
            i = citys[i].pop().unwrap();
            for k in 0..citys[i].len() {
                if citys[i][k] == j {
                    citys[i].remove(k);
                    break;
                }
            }
        }
    }

    println!("{}", res.join(" "))
}
