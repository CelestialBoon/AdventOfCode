use std::fs::File;
use std::io::{prelude::*, BufReader};
use regex::Regex;
use std::collections::HashSet;

fn main() {
    let file = File::open("inputs/i01.txt").unwrap();
    let reader = BufReader::new(file);
    let rx = Regex::new(r"([LR])(\d+)").unwrap();

    let (mut x, mut y, mut verso) = (0, 0, 0);
    let mut places:HashSet<(isize, isize)> = HashSet::new();
    places.insert((0,0));

    let mut goal:(isize, isize) = (0,0);

    rx.captures_iter(&reader.lines().next().unwrap().unwrap()).for_each(|cs| {
        let turn = &cs[1];
        let distance = &cs[2].parse::<isize>().unwrap();
        match turn {
            "L" => {verso = (verso + 1) % 4;},
            "R" => {verso = (verso + 3) % 4;},
            _ => panic!("verso sbagliato")
        }
        // println!("{}", verso);
        for i in 1..=*distance {
            match verso {
                0 => y+=1,
                1 => x-=1,
                2 => y-=1,
                3 => x+=1,
                _ => panic!("direzione sbagliata")
            };
            if !places.insert((x, y)) {
                // goal = (x, y);
                println!("{}", isize::abs(x) + isize::abs(y));
                return
            }
        }
    });
}
