use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Monkey {
    idx: usize,
    items: Vec<u128>,
    worry: (char, u128),
    test: u128,
    if_true: usize,
    if_false: usize,
    active: u128
}

#[allow(dead_code)]
fn handle11() {
    let mut monke: Vec<Monkey> = include_str!("../data1.txt")
        .split("\n\n")
        .map(|line| {
            let mut x = line.trim_start()
                .split("\n");
            Monkey { 
                idx: x.next()
                    .unwrap()
                    .split_once(" ")
                    .unwrap()
                    .1
                    .trim_end_matches(":")
                    .parse::<usize>()
                    .unwrap_or(0),
                items: x.next()
                    .unwrap()
                    .split_once(": ")
                    .unwrap()
                    .1
                    .split(", ")
                    .map(|x| {
                        x.parse::<u128>().unwrap()
                    })
                    .collect(), 
                worry: {
                    let mut temp: Vec<&str> = x.next()
                        .unwrap()
                        .split_whitespace().collect();
                    let x = temp.pop().unwrap().parse::<u128>().unwrap_or(0);
                    let y = temp.pop().unwrap().parse::<char>().unwrap();
                    (y, x)
                },
                test: x.next()
                    .unwrap()
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<u128>()
                    .unwrap(),
                if_true: x.next()
                    .unwrap()
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(), 
                if_false: x.next()
                    .unwrap()
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
                active: 0,
                }
        })
        .collect();
    for _ in 0..10000 {
        for j in 0..monke.len() {
            let (to, item) = monke[j].throw();
            for k in 0..to.len() {
                monke[j].items.clear();
                monke[to[k]].items.push(item[k]);
            }
        }
        //println!("{:?}", monke[0].active);

    }
    let mut mb: Vec<u128> = monke.iter()
        .map(|x| x.active)
        .collect();
    mb.sort();
    println!("{:?}", mb.pop().unwrap());
}

impl Monkey {
    fn throw(&mut self) -> (Vec<usize>, Vec<u128>) {
        let mut inspect: Vec<usize> = Vec::new();
        let mut item: Vec<u128> = Vec::new();
        for i in &self.items {
            self.active += 1;
            let mul = match self.worry.0 {
                '*' => {
                    match self.worry.1 {
                        0 => (i * i) / 10,
                        _ => (i * self.worry.1) / 10
                    }
                },
                '+' => {
                    match self.worry.1 {
                        0 => (i + i) / 10,
                        _ => (i + self.worry.1) / 10
                    }
                },
                _ => unreachable!()
            };
            //println!("{:?}",mul);
            match mul % self.test as u128 {
                0 => inspect.push(self.if_true),
                _ => inspect.push(self.if_false)
            };
            item.push(mul);
        }
        (inspect, item)
    }
}

fn fill_between(first: &str, second: &str, set: &mut [[bool; 200]; 1000]) {
    let first = first.split(',').map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let second = second.split(',').map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let (lower_x, higher_x) = if first[0] < second[0] { (first[0], second[0]) } else { (second[0], first[0]) };
    let (lower_y, higher_y) = if first[1] < second[1] { (first[1], second[1]) } else { (second[1], first[1]) };
    for x in lower_x..=higher_x {
        for y in lower_y..=higher_y {
            set[x][y] = true
        }
    }
}

fn simulate_sand(mut rocks: [[bool; 200]; 1000], lowest: usize, floor: bool) -> usize {
    if floor { rocks.iter_mut().for_each(|c| c[lowest + 2] = true) }
    let mut sand_count = 0;
    'sand: loop {
        let mut sand: (usize, usize) = (500, 0);
        'fall: loop {
            if !rocks[sand.0][sand.1 + 1] { sand.1 += 1 }
            else if !rocks[sand.0 - 1][sand.1 + 1] { sand.0 -= 1; sand.1 += 1 }
            else if !rocks[sand.0 + 1][sand.1 + 1] { sand.0 += 1; sand.1 += 1 }
            else if floor && sand == (500, 0) { sand_count += 1; break 'sand }
            else { rocks[sand.0][sand.1] = true; break 'fall }

            if sand.1 > lowest && !floor { break 'sand } 
        }
        sand_count += 1;
    }
    sand_count
}

fn handle14() {
    let now = std::time::Instant::now();
    let input = include_str!("../data1.txt");
    let mut rocks = [[false; 200]; 1000];

    for line in input.lines() {
        for endpoints in line.split(" -> ").collect::<Vec<&str>>().windows(2) {
            if let [first, second] = endpoints {
                fill_between(first, second, &mut rocks);
            }
        }
    }

    let lowest = rocks.iter().map(|c| c.iter().enumerate().filter(|(_, f)| **f).map(|(i, _)| i).max().unwrap_or(0)).max().unwrap();
    let part1 = simulate_sand(rocks.clone(), lowest, false);
    let part2 =  simulate_sand(rocks, lowest, true);

    println!("Part 1: {part1}, Part 2: {part2} in {:#?}", now.elapsed());
}