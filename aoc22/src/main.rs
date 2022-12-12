
#[derive(Debug)]
struct Monkey {
    idx: usize,
    items: Vec<u32>,
    worry: u32,
    test: u32,
    if_true: usize,
    if_false: usize
}

fn main() {
    let input: Vec<Monkey> = include_str!("../data1.txt")
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
                        println!("{:?}",x);
                        x.parse::<u32>().unwrap()
                    })
                    .collect(), 
                worry: x.next()
                    .unwrap()
                    .split_whitespace()
                    .last().unwrap().parse::<u32>().unwrap_or(0),
                test: x.next()
                    .unwrap()
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<u32>()
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
                }
        })
        .collect();
    println!("{:?}", input);
}
