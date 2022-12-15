use std::collections::{HashMap, HashSet};



fn main() {
    let mut cave: HashMap<u32, HashSet<u32>> = HashMap::new();
    let inp = include_str!("../data1.txt")
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
                            (start.1..=end.1).for_each(|y| {
                                cave.entry(start.0).or_default().insert(y);
                            })
                        } else {
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
}
