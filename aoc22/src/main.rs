use std::{borrow::{BorrowMut}, collections::HashMap};


pub struct Signal {
    cycles: u32,
    value: i32
}

fn parse_10(input: &str) -> Vec<Signal> {
    input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|mut line| {
            let instruction = line.next().unwrap();
            let value = match line.next() {
                Some(value) => value.parse::<i32>().unwrap_or(0_i32),
                None => 0_i32
            };
            match instruction {
                "noop" => Signal { cycles: 1, value: value },
                "addx" => Signal { cycles: 2, value: value },
                _ => panic!("invalid instruction")
            }
        })
        .collect()
}

fn main() {
    let input: &str = include_str!("../data1.txt");
    let mut crt: Vec<Vec<String>> = vec![vec![]; 6];
    let mut pixel: i32 = 0;
    let mut registry: i32 = 1;
    let mut cycle = 0;
    let mut row;
    let signals = parse_10(input);

    for signal in signals {
        for _ in 0..signal.cycles {
            row = cycle / 40;
            cycle += 1;
            if (registry - pixel).abs() <= 1 {
                crt[row].push("#".to_owned());
            } else {
                crt[row].push(".".to_owned());
            }
            pixel += 1;
            pixel %= 40;
        }
        registry += signal.value;
    };

    for row in crt {
        println!("{}", row.join(""));
    }
    
}
