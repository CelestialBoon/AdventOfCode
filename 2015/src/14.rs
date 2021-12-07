use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::cmp;
// use std::collections::HashMap;
// use itertools::Itertools;
use regex::Regex;

fn main() {
    let reader = BufReader::new(File::open("inputs/i14.txt").unwrap());
    let rex = Regex::new(r"^(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds\.$").unwrap();

    let time = 2503;
    
    #[derive(Debug)]
    struct Reindeer {
        name: String,
        vfly: isize,
        tfly: isize,
        trest: isize,
        ttotal: isize,
        flydistance: isize,
    }

    let mut reindeers: Vec<Reindeer> = Vec::new();
    
    for line in reader.lines() {
        let l = line.unwrap();
        let caps = rex.captures(&l).unwrap();
        let (reindeer, vfly, tfly, trest) = (&caps[1], 
            caps[2].parse::<isize>().unwrap(), 
            caps[3].parse::<isize>().unwrap(), 
            caps[4].parse::<isize>().unwrap());
        reindeers.push(Reindeer {
            name:reindeer.to_string(), vfly:vfly, tfly:tfly, trest:trest,
            ttotal:tfly+trest, flydistance:vfly*tfly
        });
    }
    let mut maxdistance = 0;
    for r in &reindeers {
        maxdistance = cmp::max(maxdistance, time / r.ttotal * r.flydistance + cmp::min(r.tfly, time % r.ttotal) * r.vfly);
    }
    println!("{}", maxdistance);

    let mut points= vec![0; reindeers.len()];
    let mut distance= vec![0; reindeers.len()];

    for t in 0..time {
        for (i, r) in reindeers.iter().enumerate() {
            if t % r.ttotal < r.tfly { distance[i] += r.vfly };
        }
        let maxdistance = distance.iter().max().unwrap();
        distance.iter().enumerate().filter_map(|(i, d)| (*d == *maxdistance).then(|| i)).for_each(|i| {points[i] += 1;}); 
    }
    println!("distance {:?}", distance);
    println!("points {:?}", points);
    println!("sum {}", points.iter().sum::<isize>());
    println!("maxpoints {}", points.iter().max().unwrap())
}