use std::borrow::BorrowMut;

fn main() {
    let mut sums: Vec<u32> = Vec::new();
    let mut idx: usize = 0;
    let mut max: u32 = 0;
    let mut max_idx: usize = 0;
    sums.push(0);
    let input: Vec<&str> = include_str!("../data1.txt")
        .lines()
        .collect();
    for i in input {
        if i == "" {
            if sums[idx] > max {
                max = *sums[idx].borrow_mut();
                max_idx = idx;
            }
            sums.push(0);
            idx = idx + 1;
        } else {
            sums[idx] += i.parse::<u32>().unwrap();
        }
    }
    println!("{:?}", max_idx + 1);
}
