use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;

fn main() {
    let reader = BufReader::new(File::open("inputs/i13.txt").unwrap());
    let rex = Regex::new(r"^(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+)\.$").unwrap();

    let mut prevdude:String = "".to_string();
    let mut happimap: HashMap<String, isize> = HashMap::new();
    let mut dudemap: HashMap<String, HashMap<String, isize>> = HashMap::new();
    let mut myhappimap: HashMap<String, isize> = HashMap::new();

    for line in reader.lines() {
        let l = line.unwrap();
        // println!("{}", l);
        let caps = rex.captures(&l).unwrap();
        let (dude1, gain_lose, units, dude2) = (&caps[1], &caps[2], &caps[3], &caps[4]);
        if prevdude != "" && prevdude != dude1 {
            myhappimap.insert(prevdude.clone(), 0);
            happimap.insert("me".to_string(), 0);
            dudemap.insert(prevdude, happimap);
            happimap = HashMap::new();
        }
        prevdude = dude1.to_string();
        let num = units.parse::<isize>().unwrap() * (if gain_lose == "gain" {1} else {-1});
        happimap.insert(dude2.to_string(), num);
    }
    myhappimap.insert(prevdude.clone(), 0);
    happimap.insert("me".to_string(), 0);
    dudemap.insert(prevdude, happimap);
    dudemap.insert("me".to_string(), myhappimap);

    let size = dudemap.len();
    let mut maxhappiness = 0; 
    let mut iter = dudemap.iter();
    let mut table = vec![""; size];
    table[0] = iter.by_ref().next().unwrap().0;
    for perm in iter.permutations(size - 1) {
        let mut happiness = 0;
        for (i, p) in perm.iter().enumerate() {
            table[i+1] = p.0;
        }
        for i in 0..size-1 {
            // println!("{} {} {}", i, table[i], table[i+1]);
            happiness += dudemap[table[i]][table[i+1]];
            happiness += dudemap[table[i+1]][table[i]];
        }
        happiness += dudemap[table[0]][table[size-1]];
        happiness += dudemap[table[size-1]][table[0]];
        if happiness > maxhappiness { maxhappiness = happiness; }
    }

    println!("{}", maxhappiness);
}