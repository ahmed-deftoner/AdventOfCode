use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, self};

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

fn main() {
    let mut numbers = NumVec(Vec::new());
    numbers.0 = read("/mnt/e/AdventOfCode/data.txt").unwrap();
    let n = check_window(numbers.0);
    println!("{:?}",n);
}