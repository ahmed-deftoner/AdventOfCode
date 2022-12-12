

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

#[derive(Debug, PartialEq, Clone, Copy)]
struct Coord {
    x: u32,
    y: u32
}

#[allow(dead_code)]
fn handle9() {
    /*let input: Vec<&str> = include_str!("../data1.txt")
        .lines()
        .collect();
    let mut total = 1;
    let mut h_coord: Coord = Coord { x: 0, y: 0 };
    let mut t_coord: Coord = Coord { x: 0, y: 0 }; 
    let mut visited: Vec<String> = Vec::new();
    let mut result: &String = &String::new();
    for line in input {
        let (mov, val) = line.split_once(" ")
                                         .unwrap();
        match mov {
            "R" => { 
                h_coord.x += val.parse::<u32>().unwrap(); 
            },
            "L" => {
                h_coord.x -= val.parse::<u32>().unwrap(); 
            },
            "U" => { 
                h_coord.y += val.parse::<u32>().unwrap();
            },
            "D" => { 
                h_coord.y -= val.parse::<u32>().unwrap(); 
            },
            _ => unreachable!()
        }

    }
    println!("{:?}",total); */
}

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

#[allow(dead_code)]
fn handle10() {
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

const DISK_SPACE_CAPACITY: u32 = 70_000_000;
const DISK_SPACE_NEEDED: u32 = 30_000_000;

#[derive(Debug, PartialEq)]
enum FileType {
    Directory,
    File
}

#[derive(Debug)]
struct Node {
    idx: usize,
    name: String,
    parent: Option<usize>,
    children: Vec<usize>,
    size: u32,
    file_type:FileType
}

impl Node {
    fn new(idx: usize, name: String, size: u32, file_type: FileType) -> Self {
        Self { 
            idx: idx,
            name: name,
            parent: None,
            children: vec![],
            size: size,
            file_type: file_type
        }
    }    
}

#[derive(Debug,Default)]
pub struct Tree {
    tree: Vec<Node>
}

impl Tree {
    fn add_node(&mut self, name: String, size: u32, file_type: FileType) -> usize {
        for node in &self.tree  {
            if node.name == name {
                return node.idx;
            }
        }
        let idx = self.tree.len();
        self.tree.push(Node::new(idx, name, size, file_type));
        idx
    }

    fn get_directory_size(&self, idx: usize) -> u32 {
        let mut ret = 0;
        for p in &self.tree[idx].children {
            ret += self.get_directory_size(*p);
        }
        ret + self.tree[idx].size
    }

    fn get_directories(&self) -> Vec<&Node> {
        self.tree
            .iter()
            .filter(|node| {
                node.file_type == FileType::Directory
            })
            .collect::<Vec<&Node>>()
    }
}

#[allow(dead_code)]
fn handle7() {
    let input: Vec<&str> = include_str!("../data1.txt")
    .lines()
    .skip(2)
    .collect();
    let mut tree: Tree = Tree::default();
    let root = tree.add_node("/".to_owned(), 0, FileType::Directory);
    let mut pwd = root;
    let mut current = root;
    for line in &input {
        let cmd: Vec<&str> = line.split_whitespace().collect();
        match cmd[0] {
            "dir" => {
                let dir = tree.add_node( 
                        format!("{}{}/", tree.tree[pwd].name.to_owned(), cmd[1].to_owned()),
                    0, 
                        FileType::Directory) ;
                tree.tree[dir].parent = Some(pwd);
                tree.tree[pwd].children.push(dir);
            },
            "$" => {
                if cmd[1] == "cd" {
                    if cmd[2] == ".." {
                        current = tree.tree[current].parent.unwrap();
                    } else {
                        let dir = tree.add_node( 
                            format!("{}{}/", tree.tree[pwd].name.to_owned(), cmd[2].to_owned()),
                            0, 
                            FileType::Directory);
                        current = dir;
                    }
                    pwd = current;
                }
            },
            _ => {
                let file_size = cmd[0].parse::<u32>().unwrap();
                let file = tree.add_node(
                    format!("{}{}", tree.tree[current].name, cmd[1]),
                    file_size,
                    FileType::File
                    );
                tree.tree[current].children.push(file);
                tree.tree[file].parent = Some(current);
            }
        }
    }
    let directories = tree.get_directories();
    let directory_sizes = directories.iter()
        .map(|node| {
            tree.get_directory_size(node.idx)
        })
        .filter(|size| {
            *size <= 100_000
        })
        .collect::<Vec<u32>>();
    println!("{:?}", directory_sizes.iter().sum::<u32>());

    let unused = DISK_SPACE_CAPACITY - tree.get_directory_size(0);
    let needed = DISK_SPACE_NEEDED - unused;
    let min_dir = directories.iter()
        .map(|node| {
            tree.get_directory_size(node.idx)
        })
        .filter(|size| {
            *size >= needed
        }).min().unwrap();
    println!("{:?}", min_dir);
}