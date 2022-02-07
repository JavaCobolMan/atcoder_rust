use proconio::input;

const D: usize = 998244353;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }
    let mut buff1: [usize; 10] = [0; 10];
    let mut buff2: [usize; 10] = [0; 10];
    let mut flag = true;
    buff1[an[0]] = 1;
    for i in 1..n {
        let mut tmp1 = if flag { buff1 } else { buff2 };
        let mut tmp2 = if !flag { buff1 } else { buff2 };

        for (j, &tmp) in tmp1.iter().enumerate() {
            if tmp != 0 {
                let (f, g) = get_f_g(j, an[i]);
                tmp2[f] += tmp % D;
                tmp2[g] += tmp % D;
            }
        }

        tmp1 = clear(tmp1);
        buff1 = if flag { tmp1 } else { tmp2 };
        buff2 = if !flag { tmp1 } else { tmp2 };
        flag = if flag { false } else { true };
    }
    for val in if flag { &buff1 } else { &buff2 } {
        println!("{}", val % D);
    }
}

fn get_f_g(num1: usize, num2: usize) -> (usize, usize) {
    return ((num1 + num2) % 10, (num1 * num2) % 10);
}

fn clear(mut tmp: [usize; 10]) -> [usize; 10] {
    for val in tmp.iter_mut() {
        *val = 0;
    }
    return tmp;
}
