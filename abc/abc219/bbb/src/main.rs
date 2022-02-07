use proconio::input;

fn main() {
    input! {
        mut k: usize,
        mut a: usize,
        mut b: usize,
    }
    let r = get_hex(k, a) * get_hex(k, b);
    println!("{}", r);
}

fn get_hex(k: usize, mut num: usize) -> usize {
    let mut r = 0;
    let mut i = 0;
    loop {
        if num == 0 {
            break;
        }
        let tmp = num % 10;
        r += tmp * k.pow(i);
        num /= 10;
        i += 1;
    }
    return r;
}
