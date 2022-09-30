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

fn safe_points(points: Vec<Point>) {
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
    let width = max_x;
    let height = max_y;

    let mut tempArr = vec![vec![0; width]; height];
    for i in 0..width {
        for j in 0..height {
            tempArr[i][j] = 0;
            print!("{:?} ",tempArr[i][j]);
        }
        println!();
    }
    for p in points {
        if p.x1 == p.x2 {
            
        }
    }
}

fn main() {
    let str = include_str!("../../data5.txt");
    let arr: Vec<Point> = str.lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    safe_points(arr);
}