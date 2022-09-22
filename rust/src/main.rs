use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read, self};

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

fn main() {
    let mut numbers = NumVec(Vec::new());
    numbers.0 = read("/mnt/e/AdventOfCode/data.txt").unwrap();
    print!("{numbers}");
}