use std::borrow::BorrowMut;

#[allow(dead_code)]
fn handle1() {
    let mut sums: Vec<u32> = Vec::new();
    let mut idx: usize = 0;
    let mut max: u32 = 0;
    sums.push(0);
    let input: Vec<&str> = include_str!("../data1.txt")
        .lines()
        .collect();
    for i in input {
        if i == "" {
            if sums[idx] > max {
                max = *sums[idx].borrow_mut();
            }
            sums.push(0);
            idx = idx + 1;
        } else {
            sums[idx] += i.parse::<u32>().unwrap();
        }
    }
    sums.sort();
    let total: u32 = sums.pop().unwrap() + sums.pop().unwrap() + sums.pop().unwrap();
    println!("{:?}", total);
}

fn main() {
    let mut op: Vec<u8> = Vec::new();
    let mut my: Vec<u8> = Vec::new();
    let mut total: u32 = 0;
    let input: Vec<&str> = include_str!("../data1.txt")
        .lines()
        .collect();
    for line in input {
        let (opp, myy) = line.split_once(" ").unwrap();
        let valop: u32 = match opp {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => unreachable!()
        };
        let valmy: u32 = match myy {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => unreachable!()
        };
        match valop.cmp(&valmy) {
            std::cmp::Ordering::Less => total = total + 6 + valmy,
            std::cmp::Ordering::Equal => total = total + valop + valmy,
            std::cmp::Ordering::Greater => total = total + valmy,
        }
    }
    println!("{:?}", total);
}
