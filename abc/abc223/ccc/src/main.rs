use proconio::input;

fn main() {
    input! {
        n: usize,
        mut abn: [(usize, usize); n],
    }
    let mut l_l = 0.0;
    let mut l_r;
    let mut r_l;
    let mut r_r = 0.0;
    let mut index = n / 2;
    let mut index_buf = n / 4;
    loop {
        for (a, b) in &abn[..index] {
            l_l += *a as f64 / *b as f64;
        }
        l_r = l_l + abn[index].0 as f64 / abn[index].1 as f64;
        for (a, b) in &abn[index + 1..] {
            r_r += *a as f64 / *b as f64;
        }
        r_l = r_r + abn[index].0 as f64 / abn[index].1 as f64;

        if l_l > r_l {
            index -= if index_buf / 2 == 0 {
                1
            } else {
                index_buf /= 2;
                index_buf
            };
        } else if l_l <= r_l && l_r >= r_r {
            break;
        } else {
            index += if index_buf / 2 == 0 {
                1
            } else {
                index_buf /= 2;
                index_buf
            };
        }

        l_l = 0.0;
        r_r = 0.0;
    }

    let mut buf = 0.0;
    for (a, _) in &abn[..index] {
        buf += *a as f64;
    }
    let buf_t = (l_l - r_r).abs();
    let buf_a1 = buf_t * abn[index].1 as f64;
    let buf_a2 = (abn[index].0 as f64 - buf_t * abn[index].1 as f64) / 2.0;

    let r = if l_l == r_l {
        buf
    } else if l_r == r_r {
        buf + abn[index].0 as f64
    } else if l_l > r_r {
        buf + buf_a2
    } else {
        buf + buf_a1 + buf_a2
    };

    println!("{}", r);
}
