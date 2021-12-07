use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::cmp;

fn main() {
    let file = File::open("inputs/i07.txt").unwrap();
    let reader = BufReader::new(file);

    let nums:Vec<isize> = reader.lines().next().unwrap().unwrap().split(',').map(|s| s.parse::<isize>().unwrap()).collect();
    let max = nums.iter().max().unwrap();

    let mut mindistances = isize::MAX;
    for i in 0..=*max {
        mindistances = cmp::min(mindistances, nums.iter().map(|n| {let a = isize::abs(n-i); a*(a+1)/2}).sum::<isize>());   
    }
    println!("{}", mindistances);
}
