use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("inputs/i01.txt").unwrap();
    let reader = BufReader::new(file);
    let mut (inc1, inc2) = 0;

    let mut prev1 : Option<i32> = None;
    let mut prev2 : Option<i32> = None;
    let mut prev3 : Option<i32> = None;

    for line in reader.lines() {
        let n = line.unwrap().parse::<i32>().unwrap();
        match prev1 {
            Some(p) if p < n => { inc1 += 1; }
            _ => {}
        }
        match prev3 {
            Some(p) if p < n => { inc2 += 1; }
            _ => {}
        }
        prev3 = prev2;
        prev2 = prev1;
        prev1 = Some(n);
    }
    println!("{} - {}", inc1, inc2);
}
