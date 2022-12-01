fn main() {
    let mut sums: Vec<u32> = Vec::new();
    let mut idx: usize = 0;
    sums.push(0);
    let input: Vec<&str> = include_str!("../data1.txt")
        .lines()
        .collect();
    for i in input {
        if i == "" {
            sums.push(0);
            idx = idx + 1;
        } else {
            sums[idx] += i.parse::<u32>().unwrap();
        }
    }
    println!("{:?}", sums.iter().max().unwrap());
}
