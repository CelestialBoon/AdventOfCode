use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::thread;

fn main() {
    let mut instructions:Vec<_> = Vec::new();
    
    let (mut v1, mut v2) = (0, 0);
    for (i,l) in BufReader::new(File::open("inputs/i24.txt").unwrap()).lines().map(|l| l.unwrap()).enumerate() {
        if i%18 == 4 {
            v1 = l.split(' ').skip(2).next().unwrap().parse::<i64>().unwrap();
        } else if i%18 == 5 {
            v2 = l.split(' ').skip(2).next().unwrap().parse::<i64>().unwrap();
        } else if i%18 == 15 {
            let v3 = l.split(' ').skip(2).next().unwrap().parse::<i64>().unwrap();
            instructions.push((v1, v2, v3));
        }
    }
    println!("{:?}", instructions);
    //pass it to numbers decreasing from 10^15-1 to 10^14
    let mut children:Vec<_> = Vec::new();
    for i in 1..90 {
        if i%10 == 0 {continue;}
        let is = instructions.clone();
        children.push(thread::spawn(move || {
            let mut num = 1000000000000000 - i;
            'lep: loop {
                let mut dg = DigitsBase10{num:num, mask:100000000000000}.into_iter();
                let mut z:i64 = 0;
                for (v1, v2, v3) in &is {
                    let w = dg.by_ref().next().unwrap() as i64;
                    if w == 0 {continue 'lep;}
                    let x = (((z%26)+v2) != w) as i64;
                    // println!("{}", z);
                    z = z/v1*(25*x+1) + x*(w+v3);
                }
                if z == 0 {
                    println!("{}", i);
                    break;
                }
                num -=100;
            }
        }));
    }
    for child in children {
        let _ = child.join();
    }
}

pub struct DigitsBase10 {
    num: usize,
    mask: usize
}

impl Iterator for DigitsBase10 {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.num == 0 {return None;}

        let digit = (self.num / self.mask) % 10;
        self.mask /= 10;

        Some(digit as u8)
    }
}