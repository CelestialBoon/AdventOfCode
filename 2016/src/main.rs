use std::fs::File;
use std::io::{prelude::*, BufReader};
// use std::collections::{HashMap, HashSet};
use regex::Regex;

fn main() {
    let file = File::open("inputs/i04.txt").unwrap();
    #[derive(Debug)]
    struct Room {
        text:String,
        real:bool,
        id:isize,
        checksum:String
    }
    let rx = Regex::new(r"^([\w\-]+)-(\d+)\[(\w+)\]$").unwrap();
    let alphabet:Vec<char> = (b'a'..=b'z').map(|i| i as char).collect();

    for line in BufReader::new(file).lines() {
        let l = line.unwrap();
        let cps = rx.captures(&l).unwrap();
        let mut room = Room {
            text:cps[1].to_string(),
            real:true,
            id:cps[2].parse::<isize>().unwrap(),
            checksum:cps[3].to_string()
        };
        let chars:Vec<char> = room.text.replace("-", "").chars().collect();
        let charsordered:Vec<char, isize> = alphabet.map(|l| (l, chars.iter().filter(|c| c == l).count())).collect();

        let checksum = charsordered.iter().sort_by(|(c, i), (c2, i2)| {
            let cmp = i.cmp(i2);
            if cmp != 0 {cmp}
            else {c.cmp(c2)}
        }).take(5).fold(String::new(), |acc, (c, i)| acc + c);



        println!("{}", checksum);
    }

}