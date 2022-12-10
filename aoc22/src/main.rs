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

#[allow(dead_code)]
fn handle5() {
    let (stackstr, movestr) = include_str!("../data1.txt")
        .split_once("\n\n")
        .unwrap();
    let stack_raw: Vec<&str> = stackstr.split("\n").collect();
    let moves_raw: Vec<&str> = movestr.split("\n")
        .collect();
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
    for i in moves_raw {
        let temp: Vec<&str> = i.split_whitespace()
                    .collect();
        let num = temp[1].parse::<u32>().unwrap();
        let from = temp[3].parse::<u32>().unwrap();
        let to = temp[5].parse::<u32>().unwrap();
        let mut temp_arr: Vec<char> = Vec::new();
        for _ in 0..num {
            let temp = stack[(from - 1) as usize].pop().unwrap();
            temp_arr.push(temp);
        }
        temp_arr.reverse();
        for x in &temp_arr {
            stack[(to - 1) as usize].push(*x);
        }
        temp_arr.clear();
    }
    let mut result: String = String::new();
    for i in stack.iter_mut() {
        let temp: &str = &i.pop().unwrap().to_string();
        result = format!("{}{}", result, temp);
    }
    println!("{:?}", result); 
}

#[allow(dead_code)]
fn handle6() {
    let input = include_str!("../data1.txt");
    let mut n = 14;
    for _ in 0..input.len() - 1 {
        let mut found: bool = false;
        let substr = input.get(n-14..n).unwrap();
        for c in substr.chars() {
            let temp_str = substr.replacen(c, "", 1);
            if temp_str.contains(c) {
                n += 1;
                found = false;
                break;
            } else {
                found = true;
            }
        }
        if found == true {
            println!("{:?}", n);
            break;
        }
    }
}

#[allow(dead_code)]
fn handle8() {
    let input: Vec<&str> = include_str!("../data1.txt")
        .lines()
        .collect();
    let col_size = include_str!("../data1.txt")
        .lines()
        .count();
    let mut arr: Vec<Vec<u32>> = vec![vec![]; col_size];
    let mut counter = 0;
    for line in input {
        for c in line.chars() {
            arr[counter].push(c.to_digit(10).unwrap());
        }
        counter += 1;
    }
    let mut max_score: u32 = 0;
    for i in 1..col_size-1 {
        for j in 1..arr[i].len() - 1 {
            // check up
            let mut row: i32 = (i - 1).try_into().unwrap();
            let mut up: u32 = 0;
            let mut down: u32 = 0;
            let mut left: u32 = 0;
            let mut right: u32 = 0;
            while row > -1 {
                up += 1;
                if arr[row as usize][j] >= arr[i][j] {
                    break;
                }
                row -= 1;
            }
            row = (i + 1).try_into().unwrap();
            while row < col_size.try_into().unwrap() {
                down += 1;
                if arr[row as usize][j] >= arr[i][j] {
                    break;
                }
                row += 1;
            }
            let mut col: i32 = (j + 1).try_into().unwrap();
            while col < arr[i].len().try_into().unwrap() {
                right += 1;
                if arr[i][col as usize] >= arr[i][j] {
                    break;
                }
                col += 1;
            }
            col = (j - 1).try_into().unwrap();
            while col > -1 {
                left += 1;
                if arr[i][col as usize] >= arr[i][j] {
                    break;
                }
                col -= 1;
            }
            let local_score = left * right * up * down;
            if local_score > max_score {
                max_score = local_score; 
            }
        }
    }
    println!("{:?}", max_score);
}

#[derive(Debug)]
struct Coord {
    x: u32,
    y: u32
}

fn main() {
    let input: Vec<&str> = include_str!("../data1.txt")
        .lines()
        .collect();
    let mut total = 1;
    let mut count: u32 = 0;
    let mut h_coord: Coord = Coord { x: 0, y: 0 };
    let mut t_coord: Coord = Coord { x: 0, y: 0 }; 
    for line in input {
        let (mov, val) = line.split_once(" ")
                                         .unwrap();
        match mov {
            "R" => { 
                h_coord.x += val.parse::<u32>().unwrap(); 
                if h_coord.y == t_coord.y {
                    total += h_coord.x - t_coord.x;
                } else {
                    if h_coord.x > t_coord.x + 1 {
                        total += (h_coord.x) - (t_coord.x + 1);
                        t_coord.y = h_coord.y;
                        t_coord.x = h_coord.x - 1;
                    }
                }
            },
            "L" => { h_coord.x -= val.parse::<u32>().unwrap(); },
            "U" => { h_coord.y += val.parse::<u32>().unwrap(); },
            "D" => { h_coord.y -= val.parse::<u32>().unwrap(); },
            _ => unreachable!()
        }

    }
    println!("{:?}",total);
}
