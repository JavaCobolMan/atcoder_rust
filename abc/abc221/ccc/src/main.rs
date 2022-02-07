use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }
    let mut tmp_nums1: Vec<usize> = vec![0; 10];
    while n != 0 {
        let tmp1 = n % 10;
        tmp_nums1[tmp1] += 1;
        n /= 10;
    }
    let mut tmp_nums2: Vec<usize> = vec![];
    for i in &[9, 8, 7, 6, 5, 4, 3, 2, 1, 0] {
        for _ in 0..tmp_nums1[*i] {
            tmp_nums2.push(*i);
        }
    }
    let mut tmp1 = *tmp_nums2.get(0).unwrap();
    let mut tmp2 = *tmp_nums2.get(1).unwrap();
    let mut cnt = 2;
    loop {
        if cnt == tmp_nums2.len() {
            break;
        } else if cnt + 1 == tmp_nums2.len() {
            if (tmp1 * 10 + tmp_nums2[cnt]) * tmp2 > tmp1 * (tmp2 * 10 + tmp_nums2[cnt]) {
                tmp1 = tmp1 * 10 + tmp_nums2[cnt];
            } else {
                tmp2 = tmp2 * 10 + tmp_nums2[cnt];
            }
            break;
        } else {
            if (tmp1 * 10 + tmp_nums2[cnt]) * (tmp2 * 10 + tmp_nums2[cnt + 1])
                > (tmp1 * 10 + tmp_nums2[cnt + 1]) * (tmp2 * 10 + tmp_nums2[cnt])
            {
                tmp1 = tmp1 * 10 + tmp_nums2[cnt];
                tmp2 = tmp2 * 10 + tmp_nums2[cnt + 1];
            } else {
                tmp1 = tmp1 * 10 + tmp_nums2[cnt + 1];
                tmp2 = tmp2 * 10 + tmp_nums2[cnt];
            }
            cnt += 2;
        }
    }
    println!("{}", tmp1 * tmp2);
}
