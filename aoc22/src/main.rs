use std::{i32, collections::HashSet};

#[derive(Debug)]
struct Sensor {
    x: i32,
    y: i32,
    radius: i32
}

#[derive(Debug)]
struct Beacon {
    x: i32,
    y: i32
}


fn main() {
    let y = 2_000_000;
    let mut not_allowed: HashSet<(i32, i32)> = HashSet::new();
    let terrain: Vec<(Sensor, Beacon)> = include_str!("../data1.txt")
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
            let beacon: Beacon = Beacon {
                x: bx.replace("x=", "").parse::<i32>().unwrap(),
                y: by.replace("y=", "").parse::<i32>().unwrap(),
            };
            let mut sensor: Sensor = Sensor {
                x: sx.replace("x=", "").parse::<i32>().unwrap(),
                y: sy.replace("y=", "").parse::<i32>().unwrap(),
                radius: 1,
            };
            sensor.radius = beacon.x.abs_diff(sensor.x) as i32 + beacon.y.abs_diff(sensor.y) as i32;
            (sensor, beacon)
        })
        .collect::<Vec<(Sensor, Beacon)>>();
    terrain.iter()
        .for_each(|t| {
            let min_x: i32 = t.0.x - (t.0.radius - t.0.y.abs_diff(y) as i32);
            let max_x: i32 = t.0.x + (t.0.radius - t.0.y.abs_diff(y) as i32);
            (min_x..=max_x).for_each(|x| {
                not_allowed.insert((x, y));
            })
        });
    let existing_beacon_count = terrain
        .iter()
        .map(|(_, beacon)| (beacon.x, beacon.y))
        .collect::<HashSet<(i32, i32)>>()
        .iter()
        .filter(|(x, y)| y == y && not_allowed.contains(&(*x, *y)))
        .count();
    println!("{:?}", not_allowed.len() as u32 - existing_beacon_count as u32);
}
