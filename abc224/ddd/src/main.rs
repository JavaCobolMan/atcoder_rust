use proconio::input;

fn main() {
    input! {
        m: usize,
        mut uv: [(usize, usize); m],
        mut pp: [usize; 8],
    }

    let mut p_ini: Vec<usize> = Vec::new();
    for ppi in pp {
        p_ini.push(ppi);
    }
    let end: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let mut p_vec_vec = Vec::new();
    let mut p_vec = Vec::new();
    p_vec.push(p_ini.clone());
    p_vec_vec.push(p_vec);

    let mut p_all = Vec::new();
    p_all.push(p_ini.clone());

    'outer: for i in 0.. {
        let p_vec_optin = p_vec_vec.get(i);
        match p_vec_optin {
            None => {
                println!("-1");
                break 'outer;
            }
            Some(p_vec) => {
                let mut p_vec_next = Vec::new();
                for p_element in p_vec {
                    if *p_element == end {
                        println!("{}", i);
                        break 'outer;
                    } else {
                        for (u, v) in &uv {
                            let p_next = nextp(p_element, *u, *v);
                            match p_next {
                                None => {}
                                Some(p_tmp) => {
                                    if !p_all.contains(&p_tmp) {
                                        p_all.push(p_tmp.clone());
                                        p_vec_next.push(p_tmp);
                                    }
                                }
                            }
                            let p_next = nextp(p_element, *v, *u);
                            match p_next {
                                None => {}
                                Some(p_tmp) => {
                                    if !p_all.contains(&p_tmp) {
                                        p_all.push(p_tmp.clone());
                                        p_vec_next.push(p_tmp);
                                    }
                                }
                            }
                        }
                    }
                }
                if p_vec_next.len() != 0 {
                    p_vec_vec.push(p_vec_next);
                }
            }
        }
    }
}

fn nextp(p: &Vec<usize>, u: usize, v: usize) -> Option<Vec<usize>> {
    let mut p_tmp = p.clone();
    if p.contains(&u) {
        if p.contains(&v) {
            None
        } else {
            for pu in p_tmp.iter_mut() {
                if pu == &u {
                    *pu = v.clone();
                    break;
                }
            }
            Some(p_tmp)
        }
    } else {
        None
    }
}
