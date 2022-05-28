use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
        t: [Chars; n],
    }

    let mut s: Vec<Vec<char>> = s;
    let t: Vec<Vec<char>> = t;
    let mut s_cnt: isize = 0;
    let mut t_cnt: isize = 0;
    for i in 0..n {
        s_cnt += s[i]
            .iter()
            .map(|a| if *a == '#' { 1 } else { 0 })
            .sum::<isize>();
        t_cnt += t[i]
            .iter()
            .map(|a| if *a == '#' { 1 } else { 0 })
            .sum::<isize>();
    }
    if s_cnt != t_cnt {
        println!("{}", "No");
        return;
    }

    let mut f = false;
    for _ in 0..4 {
        let mut s_tmp: Vec<Vec<char>> = s.clone();
        for i in 0..n {
            for j in 0..n {
                s_tmp[n - i - 1][j] = s[j][i];
            }
        }

        f = check_grid(&s, &t);
        if f {
            break;
        }
        s = s_tmp;
    }

    println!("{}", if f { "Yes" } else { "No" });
}

fn check_grid(s: &Vec<Vec<char>>, t: &Vec<Vec<char>>) -> bool {
    let mut f = true;
    let s_offset = get_left_top(s);
    let t_offset = get_left_top(t);
    let i_offset = s_offset.0 - t_offset.0;
    let j_offset = s_offset.1 - t_offset.1;
    'outer: for i in 0..s.len() {
        for j in 0..s.len() {
            if t[i][j] != get_ele(s, i as isize + i_offset, j as isize + j_offset) {
                f = false;
                break 'outer;
            }
        }
    }

    return f;
}

fn get_left_top(st: &Vec<Vec<char>>) -> (isize, isize) {
    let mut tmp1: isize = 0;
    let mut tmp2: isize = 0;
    for (i, val1) in st.iter().enumerate() {
        for (j, val2) in val1.iter().enumerate() {
            if *val2 == '#' {
                tmp1 = i as isize;
                tmp2 = j as isize;
            }
        }
    }
    return (tmp1, tmp2);
}

fn get_ele(st: &Vec<Vec<char>>, i: isize, j: isize) -> char {
    let mut tmp: char = '.';
    if 0 <= i && i < st.len() as isize && 0 <= j && j < st.len() as isize {
        tmp = st[i as usize][j as usize];
    }
    return tmp;
}
