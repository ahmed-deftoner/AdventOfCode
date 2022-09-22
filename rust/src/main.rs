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

fn main() {
    let mut vec = read("../../data.txt").unwrap();
    print!("{vec}");
}