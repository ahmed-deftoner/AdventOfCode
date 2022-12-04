use std::{borrow::BorrowMut, collections::HashMap};

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

#[allow(dead_code)]
fn handle2() {
    let mut total: u32 = 0;
    let mut losses = HashMap::new();
    losses.insert(1, 3);
    losses.insert(2, 1);
    losses.insert(3, 2);
    let mut wins = HashMap::new();
    wins.insert(1, 2);
    wins.insert(2, 3);
    wins.insert(3, 1);
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
        if valmy == 1 {
            total += losses[&valop];
        } else if valmy == 2 {
            total += valop + 3;  
        } else {
            total += wins[&valop] + 6;
        }
    }
    println!("{:?}", total);
}

fn main() {
    let input: Vec<&str> = include_str!("../data1.txt")
        .lines()
        .collect();
    for line in input {
        let (first, second) = line.split_at(line.chars().count()/2);
        println!("{:?},{:?}", first, second);
    }
}
