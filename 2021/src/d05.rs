use std::fs::File;
use std::io::{prelude::*, BufReader};
// use std::collections::HashMap;
// use std::collections::VecDeque;
use regex::Regex;

fn main() {
    let reader = BufReader::new(File::open("inputs/i05.txt").unwrap());
    let rex = Regex::new(r"^([0-9]+),([0-9]+) -> ([0-9]+),([0-9]+)$").unwrap();
    let mut vents = vec![vec![0u8; 1000]; 1000];

    for line in reader.lines() {
        let l = line.unwrap();
        let caps = rex.captures(&l).unwrap();
        let nums: Vec<isize> = caps.iter().skip(1).map(|s| s.unwrap().as_str().parse::<isize>().unwrap()).collect();
        let (x1, y1, x2, y2) = (nums[0], nums[1], nums[2], nums[3]);
        if x1 != x2 && y1 != y2 { 
            let (i1, i2, j1, j2) = if x1 < x2 {(x1, x2, y1, y2)} else {(x2, x1, y2, y1)};
            let jstep = if j1 < j2 {1} else {-1};
            let mut j = j1;
            for i in i1..=i2 {
                vents[i as usize][j as usize] += 1;
                j += jstep;
            }
        } else if x1 == x2 {
            for i in if y1 < y2 { y1..=y2 } else { y2..=y1 } {
                vents[x1 as usize][i as usize] += 1;
            }
        } else if y1 == y2 {
            for i in if x1 < x2 { x1..=x2 } else { x2..=x1 } {
                vents[i as usize][y1 as usize] += 1;
            }
        } else {panic!("errore nel parse somehow")}
    }
    let mut count = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if vents[x][y] >=2 {count +=1;}
        }
    }
    println!("{}", count);
}