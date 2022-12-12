

#[derive(Debug)]
struct Monkey {
    idx: usize,
    items: Vec<u32>,
    worry: (char, u32),
    test: u32,
    if_true: usize,
    if_false: usize
}

impl Monkey {
    fn throw(&mut self) -> (Vec<usize>, Vec<u32>) {
        let mut inspect: Vec<usize> = Vec::new();
        let mut item: Vec<u32> = Vec::new();
        for i in &self.items {
            let mul = match self.worry.0 {
                '*' => {
                    match self.worry.1 {
                        0 => (i * i) / 3,
                        _ => (i * self.worry.1) / 3
                    }
                },
                '+' => {
                    match self.worry.1 {
                        0 => (i + i) / 3,
                        _ => (i + self.worry.1) / 3
                    }
                },
                _ => unreachable!()
            };
            match mul % self.test {
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
                        x.parse::<u32>().unwrap()
                    })
                    .collect(), 
                worry: {
                    let mut temp: Vec<&str> = x.next()
                         .unwrap()
                         .split_whitespace().collect();
                    let x = temp.pop().unwrap().parse::<u32>().unwrap_or(0);
                    let y = temp.pop().unwrap().parse::<char>().unwrap();
                    (y, x)
                },
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
    for _ in 0..2 {
        for j in 0..monke.len() {
            let (to, item) = monke[j].throw();
            for k in 0..to.len() {
                monke[j].items.clear();
                monke[to[k]].items.push(item[k]);
            }
        }
        println!("{:?}", monke[0].items);
    }
}
