use std::fs::File;
use std::io::{prelude::*, BufReader};
use regex::Regex;

fn main() {
    let file = File::open("inputs/i02.txt").unwrap();
    let reader = BufReader::new(file);

    let (mut length, mut aim, mut depth) = (0,0,0); 

    let forw = Regex::new(r"^forward ([0-9]+)$").unwrap();
    let up = Regex::new(r"^up ([0-9]+)$").unwrap();
    let down = Regex::new(r"^down ([0-9]+)$").unwrap();

    for line in reader.lines() {
        let l = line.unwrap();
        if forw.is_match(&l) {
            let len = forw.captures(&l).unwrap()[1].parse::<i32>().unwrap();
            length += len;
            depth += aim * len;
        } else if up.is_match(&l) {
            aim -= up.captures(&l).unwrap()[1].parse::<i32>().unwrap();
        } else if down.is_match(&l) {
            aim += down.captures(&l).unwrap()[1].parse::<i32>().unwrap();
        } else { panic!("couldn't match") }
    }
    println!("{} - {}", length*aim, length*depth);
    
}