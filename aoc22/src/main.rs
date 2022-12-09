use std::{borrow::BorrowMut, collections::HashMap, mem::replace};

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

#[allow(dead_code)]
fn handle3() {
    let input: Vec<&str> = include_str!("../data1.txt")
    .lines()
    .collect();
    let mut total: u32 = 0;
    let mut group: Vec<&str> = Vec::new();
    let mut counter = 0;
    for line in input {
        group.push(line);
        counter+=1;
        if counter == 3 {
            let mut repeat: Vec<char> = Vec::new();
            for i in group[0].chars() {
                let x = group[1].find(i);
                let y = group[2].find(i);
                if x != None && y != None && !repeat.contains(&i) {
                    repeat.push(i);
                    if i.is_lowercase() {
                        total += (i as u32) - 96;
                    } else {
                        total += (i as u32) - 38;
                    }
                }
            }    
            counter = 0;
            group.clear();
            repeat.clear();
        }
    }
    println!("{:?}", total);
}

#[allow(dead_code)]
fn handle4() {
    let input: Vec<&str> = include_str!("../data1.txt")
        .lines()
        .collect();
    let mut count = 0;
    for line in input {
        let (a1,b1) = line.split_once(",").unwrap()
            .0
            .split_once("-")
            .unwrap();
        let (x1,y1) = line.split_once(",").unwrap()
            .1
            .split_once("-")
            .unwrap();

        let a = a1.parse::<u32>().unwrap();
        let b = b1.parse::<u32>().unwrap();
        let x = x1.parse::<u32>().unwrap();
        let y = y1.parse::<u32>().unwrap();
        if b >= x && a <= y || a <= y && b >= x {
            count += 1;
          //  println!("{:?},{:?},{:?},{:?}-1",a,b,x,y);
        } 
    }
    println!("{:?}", count); 
}

fn main() {
    let (stackstr, movestr) = include_str!("../data1.txt")
        .split_once("\n\n")
        .unwrap();
    let stack_raw: Vec<&str> = stackstr.split("\n").collect();
    let moves: Vec<&str> = movestr.split("\n").collect();
    let arr_size: usize = stack_raw.last()
        .unwrap()
        .split_at(stack_raw.last().unwrap().len()-2)
        .1
        .trim_end()
        .parse::<usize>()
        .unwrap();
    let mut stack: Vec<Vec<char>> = vec![vec![]; arr_size]; 
    let mut stack_count: i32 = (stack_raw.len() - 2 ).try_into().unwrap();
    while stack_count != -1 {
        let mut arr_count = 0;
        let last_iter = stack_raw.last().unwrap().chars().enumerate();
        for (idx, c) in last_iter {
            if c != ' ' {
                let x = stack_raw[stack_count as usize].chars().nth(idx).unwrap();
                if x != ' ' {
                    stack[arr_count].push(x);                   
                }
                arr_count += 1; 
            }
        }   
        stack_count -= 1;
    }
    println!("{:?}", stack);
}
