use proconio::input;

const NUM_10: usize = 10;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
        mut x: usize,
    }
    let tmp_sum: usize = an.iter().sum();
    let mut digit1 = 0;
    let mut digit2 = 0;
    let mut cnt = 0;
    loop {
        for i in 0..=18 {
            if tmp_sum >= NUM_10.pow(i) {
                digit1 = i;
            }
            if x >= NUM_10.pow(i) {
                digit2 = i;
            }
        }

        if digit2 as isize - digit1 as isize - 1 > 0 {
            cnt += NUM_10.pow(digit2 - digit1 - 1) * an.len();
            x -= NUM_10.pow(digit2 - digit1 - 1) * tmp_sum;
        } else {
            break;
        }
    }
    let r = loop {
        if an[cnt % an.len()] > x {
            break cnt;
        }
        x -= an[cnt % an.len()];
        cnt += 1;
    };
    println!("{}", r + 1);
}
