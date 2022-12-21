use std::{i64, collections::HashSet};

#[derive(Debug)]
struct Sensor {
    x: i64,
    y: i64,
    radius: i64
}

#[derive(Debug)]
struct Beacon {
    x: i64,
    y: i64
}


fn main() {
    let y = 2_000_000;
    let mut not_allowed: HashSet<(i64, i64)> = HashSet::new();
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
                x: bx.replace("x=", "").parse::<i64>().unwrap(),
                y: by.replace("y=", "").parse::<i64>().unwrap(),
            };
            let mut sensor: Sensor = Sensor {
                x: sx.replace("x=", "").parse::<i64>().unwrap(),
                y: sy.replace("y=", "").parse::<i64>().unwrap(),
                radius: 1,
            };
            sensor.radius = beacon.x.abs_diff(sensor.x) as i64 + beacon.y.abs_diff(sensor.y) as i64;
            (sensor, beacon)
        })
        .collect::<Vec<(Sensor, Beacon)>>();
    terrain.iter()
        .for_each(|t| {
            let min_x: i64 = t.0.x - (t.0.radius - t.0.y.abs_diff(y) as i64);
            let max_x: i64 = t.0.x + (t.0.radius - t.0.y.abs_diff(y) as i64);
            (min_x..=max_x).for_each(|x| {
                not_allowed.insert((x, y));
            })
        });
    let existing_beacon_count = terrain
        .iter()
        .map(|(_, beacon)| (beacon.x, beacon.y))
        .collect::<HashSet<(i64, i64)>>()
        .iter()
        .filter(|(x, y)| y == y && not_allowed.contains(&(*x, *y)))
        .count();
    println!("{:?}", not_allowed.len() as u32 - existing_beacon_count as u32);
    
    let sensors = terrain
        .iter()
        .map(|(sensor, _)| sensor)
        .collect::<Vec<&Sensor>>();
    for sensor in sensors.iter() {
        for x in (sensor.x - sensor.radius - 1)..=(sensor.x + sensor.radius + 1) {
            if x > 4_000_000 {
                break;
            } else if x < 0 {
                continue;
            }

            let delta_y = sensor.radius - (x - sensor.x).abs() + 1;
            'a: for y in [sensor.y + delta_y, sensor.y - delta_y] {
                if y <= 4_000_000 && y >= 0 {
                    for adjacent_sensor in sensors.iter() {
                        if (adjacent_sensor.x - x).abs() + (adjacent_sensor.y - y).abs() <= adjacent_sensor.radius {
                            break 'a;
                        }
                    }
                    println!("{:?}",x * 4_000_000 + y);
                }
            }
        }
    }
}
