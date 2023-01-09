use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

fn main() {
    let mut iter = BufReader::new(File::open("inputs/i20.txt").unwrap()).lines().map(|l| l.unwrap());

    let decoder:Vec<bool> = iter.by_ref().next().unwrap().chars().map(|c| c == '#').collect();
    let _ = iter.by_ref().next();
    
    let mut map:HashMap<(isize, isize), bool> = HashMap::new();
    let (mut height, mut length):(isize, isize) = (0,0);
    for (j, l) in iter.enumerate() {
        if length == 0 { length = l.len() as isize; }
        height = j as isize;
        l.chars().enumerate().for_each(|(i, c)| {map.insert((i as isize, j as isize), c == '#');});
    }
    height+=1;
    for n in 0isize..50 {
        let default = n%2==1;
        let mut newmap:HashMap<_, _> = HashMap::new();
        for x in (-1-n)..(length+1+n) {
            for y in (-1-n)..(height+1+n) {
                let s:String = (-1..=1).map(|j| (-1..=1).map(move |i| (x+i, y+j))).flatten()
                    .map(|(i, j)| if *map.get(&(i, j)).unwrap_or(&default) {'1'} else {'0'}).collect();
                let num = usize::from_str_radix(&s, 2).unwrap();
                newmap.insert((x, y), decoder[num]);
            }
        }
        map = newmap;
    }
    println!("{}", map.iter().filter(|(_, v)| **v).count());
}