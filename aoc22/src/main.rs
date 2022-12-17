use std::i32;

#[derive(Debug)]
struct Sensor {
    x: i32,
    y: i32,
    radius: u32
}

#[derive(Debug)]
struct Beacon {
    x: i32,
    y: i32
}

fn main() {
    let inp: Vec<&str> = include_str!("../data1.txt")
        .split("\n")
        .map(|line| {
            let (s, b) = line.split_once(":").unwrap();
            let (sx, sy) = s.split_once("Sensor at ")
                .unwrap()
                .1
                .split_once(", ")
                .unwrap();
            let (bx, by) = b.split_once(" closest beacon is at ")
                .unwrap()
                .1
                .split_once(", ")
                .unwrap();
            let beacon = Beacon {
                x: bx.replace("x=", "").parse::<i32>().unwrap(),
                y: by.replace("y=", "").parse::<i32>().unwrap(),
            };
            let mut sensor = Sensor {
                x: sx.replace("x=", "").parse::<i32>().unwrap(),
                y: sy.replace("y=", "").parse::<i32>().unwrap(),
                radius: 1,
            };
            sensor.radius = beacon.x.abs_diff(sensor.x) + beacon.y.abs_diff(sensor.y);
            println!("{:?}", sensor);
            sy
        })
        .collect();
}
