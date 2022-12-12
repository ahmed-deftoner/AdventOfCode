

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

impl Monkey {
    fn throw(&mut self) -> (Vec<usize>, Vec<u128>) {
        let mut inspect: Vec<usize> = Vec::new();
        let mut item: Vec<u128> = Vec::new();
        for i in &self.items {
            self.active += 1;
            let mul = match self.worry.0 {
                '*' => {
                    match self.worry.1 {
                        0 => (i * i),
                        _ => (i * self.worry.1)
                    }
                },
                '+' => {
                    match self.worry.1 {
                        0 => (i + i),
                        _ => (i + self.worry.1)
                    }
                },
                _ => unreachable!()
            };
            match mul % self.test as u128 {
                0 => inspect.push(self.if_true),
                _ => inspect.push(self.if_false)
            };
            item.push(mul);
        }
        (inspect, item)
    }
}

fn main() {
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
        println!("{:?}", monke[0].active);

    }
    let mut mb: Vec<u128> = monke.iter()
        .map(|x| x.active)
        .collect();
    mb.sort();
    println!("{:?}", mb.pop().unwrap());
}
