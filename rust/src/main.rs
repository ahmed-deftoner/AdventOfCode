use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, self};
use std::str::FromStr;

fn read(path : &str) -> Result<Vec<i64>, io::Error>{
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut v = Vec::new();
    for line in br.lines() {
        let line = line?;
        let n = line.trim()
        .parse()
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?; 
        v.push(n);
    }
    Ok(v)
}


struct NumVec(Vec<i64>);


impl Display for NumVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = &self.0;
        if v.len() == 0 {
            return Ok(());
        }
        for num in &v[0..v.len() - 1] {
            if let Err(e) = write!(f, "{}, ", &num.to_string()) {
                return Err(e);
            }
        }
        write!(f, "{}", &v[v.len() - 1])
    }
}

fn check_increasing(x: Vec<i64>) -> u64{
    let mut count = 0;
    for i in 1..x.len() {
        if x[i-1] < x[i] {
            count=count+1;
        }
    }
    count
}

fn check_window(x: Vec<i64>) -> u64 {
    let mut sum_arr = Vec::new();
    let mut sum = 0;
    let mut counter = 0;
    let mut i = 0;
    while i < x.len() {
        if counter < 3 {
            sum += x[i];
            counter+=1;
            i+=1;
        }else if counter == 3 {
            sum_arr.push(sum);
            counter = 0;
            sum = 0;
            i-=2;
        }
    }
    println!("{:?}",sum_arr);
    return check_increasing(sum_arr);
}

#[allow(dead_code)]
fn handle1() {
    let mut numbers = NumVec(Vec::new());
    numbers.0 = read("/mnt/e/AdventOfCode/data.txt").unwrap();
    let n = check_window(numbers.0);
    println!("{:?}",n);  
}

// Day 5a
#[derive(Debug,Clone)]
struct Point {
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize
}

impl FromStr for Point {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(" -> ").unwrap();
        let (x1,y1) = x.split_once(",").unwrap();
        let (x2,y2) = y.split_once(",").unwrap();
        return Ok(Point{
            x1: x1.parse().unwrap(),
            y1: y1.parse().unwrap(),
            x2: x2.parse().unwrap(),
            y2: y2.parse().unwrap()
        });
    }
}

fn safe_points(points: Vec<Point>) -> usize {
    let mut arr = points.clone();
    let max_x: usize;
    let max_y: usize;
    arr.sort_by_key(|d| d.x1);
    let max_x1 = arr[arr.len()-1].x1;
    arr.sort_by_key(|d| d.x2);
    let max_x2 = arr[arr.len()-1].x2;
    if max_x1 >= max_x2 {
        max_x = max_x1;
    } else {
        max_x = max_x2;
    }
    arr.sort_by_key(|d| d.y1);
    let max_y1 = arr[arr.len()-1].y1;
    arr.sort_by_key(|d| d.y2);
    let max_y2 = arr[arr.len()-1].y2;
    if max_y1 >= max_y2 {
        max_y = max_y1;
    } else {
        max_y = max_y2;
    }
    println!("{:?}:{:?}",max_x,max_y);
    let width = max_x+2;
    let height = max_y+3;

    let mut temp_arr = vec![vec![0; width]; height];
    for i in 0..width {
        for j in 0..height {
            temp_arr[i][j] = 0;
        }
    }
    for p in points {
        // Day 5b
        if p.x2 > p.x1 && p.y2 > p.y1 {
            let a = p.x2 - p.x1;
            let b = p.y2 - p.y1;
            if a == b {
                for i in 0..a+1 {
                    temp_arr[p.y1+i][p.x1+i] += 1;
                }
            }   
        } else if p.x1 > p.x2 && p.y1 > p.y2 {
            let a = p.x1 - p.x2;
            let b = p.y1 - p.y2;
            if a == b {
                for i in 0..a+1 {
                    temp_arr[p.y2+i][p.x2+i] += 1;
                }
            }   
        } else if p.x1 > p.x2 && p.y1 < p.y2 {
            let a = p.x1 + p.y1;
            let b = p.x2 + p.y2;
            if a == b {
                let c = p.y2 - p.y1;
                for i in 0..c+1 {
                    temp_arr[p.y1+i][p.x1-i] += 1;
                }
            }  
        } else if p.x2 > p.x1 && p.y2 < p.y1 {
            let a = p.x1 + p.y1;
            let b = p.x2 + p.y2;
            if a == b {
                let c = p.y1 - p.y2;
                for i in 0..c+1 {
                    temp_arr[p.y2+i][p.x2-i] += 1;
                }
            }  
        } 
        // Day 5a
        else if p.x1 == p.x2 {
            if p.y1 < p.y2 {
                for i in p.y1..p.y2+1 {
                    temp_arr[i][p.x1] += 1;
                }
            }else {
                for i in p.y2..p.y1+1 {
                    temp_arr[i][p.x1] += 1;
                }
            }
        }
        else if p.y1 == p.y2 {
            if p.x1 < p.x2 {
                for i in p.x1..p.x2+1 {
                    temp_arr[p.y1][i] += 1;
                }
            }else {
                for i in p.x2..p.x1+1 {
                    temp_arr[p.y1][i] += 1;
                }
            }
        }
    }

    let mut count = 0;
/*     for i in 0..width {
        for j in 0..height {
            print!("{:?} ",temp_arr[i][j]);
        }
        println!();
    }*/
    for i in 0..width {
        for j in 0..height {
            if temp_arr[i][j] >= 2 {
                count += 1;
            }
        }
    }
    count
}

#[allow(dead_code)]
fn handle5() {
    let str = include_str!("../../data5.txt");
    let arr: Vec<Point> = str.lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    println!("{:?}",safe_points(arr));
}

#[allow(dead_code)]
fn handle6() {
    let arr: Vec<u64> = include_str!("../../data6.txt")
        .lines()
        .take(1)
        .flat_map(|x| x.split(","))
        .map(|x| x.parse::<u64>().unwrap() )
        .collect();
    let mut temp = vec![0; 9];
    for i in arr {
        temp[i as usize] += 1;
    }
    for _ in 0..256 {
        let d = temp[0];
        temp[0] = 0;
        for j in 1..9 {
            temp[j-1] = temp[j];
            temp[j] = 0;
        }
        temp[6] += d;
        temp[8] = d;
    }
    let sum: u64 = temp.iter().sum();
    println!("{:?}", sum);  
}

fn main() {
    let mut arr: Vec<u64> = include_str!("../../data7.txt")
        .lines()
        .flat_map(|x| x.split(","))
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    arr.sort();

    println!("{:?}",arr[arr.len()-1]);
}