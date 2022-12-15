

fn main() {
    let inp = include_str!("../data1.txt")
        .split("\n")
        .for_each(|line| {
            line.split(" -> ")
                .map(|pair| {
                    let (x, y) = pair.split_once(",").unwrap();
                    println!("{:?} -> {:?}",x, y);
                    (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())
                 })
                 .collect::<Vec<(u32,u32)>>();
        });
}
