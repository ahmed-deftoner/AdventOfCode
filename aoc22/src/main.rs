use std::collections::{HashMap, HashSet};



fn main() {
    let mut x = 500;
    let mut y = 0;
    let mut sand = 0;
    let mut max_y = 0;
    let mut cave: HashMap<u32, HashSet<u32>> = HashMap::new();
    include_str!("../data1.txt")
        .split("\n")
        .for_each(|line| {
            line.split(" -> ")
                .map(|pair| {
                    let (x, y) = pair.split_once(",").unwrap();
                    println!("{:?} -> {:?}",x, y);
                    (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())
                 })
                 .collect::<Vec<(u32,u32)>>()
                 .windows(2)
                 .into_iter()
                 .for_each(|window| {
                    let start = window[0];
                    let end = window[1];
                    if start.0 == end.0 {
                        if end.1 > start.1 {
                            if end.1 > max_y {
                                max_y = end.1;
                            }
                            (start.1..=end.1).for_each(|y| {
                                cave.entry(start.0).or_default().insert(y);
                            })
                        } else {
                            if start.1 > max_y {
                                max_y = start.1;
                            }
                            (end.1..=start.1).for_each(|y| {
                                cave.entry(start.0).or_default().insert(y);
                            })
                        }
                    } else if start.1 == end.1 {
                        if start.0 > end.0 {
                            (end.0..=start.0).for_each(|x| {
                                cave.entry(x).or_default().insert(start.1);                    
                            })
                        } else {
                            (start.0..=end.0).for_each(|x| {
                                cave.entry(x).or_default().insert(start.1);               
                            })
                        }
                    }
                 })
        });
    loop {
        if cave.get(&x).is_some() && cave.get(&x).unwrap().get(&(y + 1)).is_some() {
            if cave.get(&(x - 1)).is_none() && cave.get(&(x - 1)).unwrap().get(&(y + 1)).is_some() {
                if cave.get(&(x + 1)).is_some() && cave.get(&(x + 1)).unwrap().get(&(y + 1)).is_some() {
                    sand += 1;
                    if x == 500 && y == 1 {
                        break;
                    }
                    cave.get_mut(&x).unwrap().insert(y);
                    x = 500;
                    y = 0;
                } else {
                    x += 1;
                    y += 1;
                }
            } else {
                x -= 1;
                y += 1;
            }
        } else {
            y += 1;
        }
    }
    println!("{:?}", max_y);
}
