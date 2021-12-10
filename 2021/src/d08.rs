use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::{HashMap, HashSet};

fn main() {
    let file = File::open("inputs/i08.txt").unwrap();

    let mut digitslist:Vec<(Vec<String>, Vec<String>)> = Vec::new();

    for line in BufReader::new(file).lines() {
        let l:Vec<String> = line.unwrap().split_whitespace().map(|s| s.to_string()).collect();

        let zeronine:Vec<String> = l.iter().take(10).map(|s| s.to_string()).collect();
        let four:Vec<String> = l.iter().skip(11).take(4).map(|s| s.to_string()).collect();
        digitslist.push((zeronine, four));
    }

    println!("{}",  digitslist.iter().map(|(_, four)| {
        four.iter().filter(|s| {
            let len = s.len();
            len == 2 || len == 3 || len == 4 || len == 7
        }).count() as isize
    }).sum::<isize>());

    let mut sum = 0;
    for (zn, output) in digitslist {
        let mut digitmap:HashMap<usize, HashSet<char>> = HashMap::new();
        let mut segmentmap:HashMap<char, char> = HashMap::new();
        let mut zerosixnine:Vec<HashSet<char>> = Vec::new();
        let mut twothreefive:Vec<HashSet<char>> = Vec::new();
        zn.into_iter().for_each(|s| {
            let chars:HashSet<char> = s.chars().collect();
            match s.len() {
                2 => {digitmap.insert(1, chars);}
                3 => {digitmap.insert(7, chars);}
                4 => {digitmap.insert(4, chars);}
                5 => {twothreefive.push(chars);}
                6 => {zerosixnine.push(chars);}
                7 => {digitmap.insert(8, chars);}
                _ => panic!("no good number of stanghe")
            }
        });
        zerosixnine.into_iter().for_each(|dg| {
            if digitmap[&4].difference(&dg).count() == 0 {
                digitmap.insert(9, dg);
            } else if digitmap[&7].difference(&dg).count() == 0 {
                digitmap.insert(0, dg);
            } else {
                digitmap.insert(6, dg);
            }
        });
        segmentmap.insert('a', *digitmap[&7].difference(&digitmap[&1]).next().unwrap());
        segmentmap.insert('e', *digitmap[&8].difference(&digitmap[&9]).next().unwrap());
        segmentmap.insert('c', *digitmap[&8].difference(&digitmap[&6]).next().unwrap());
        segmentmap.insert('d', *digitmap[&6].difference(&digitmap[&0]).next().unwrap());
        // {
        //     let mut oneclone = digitmap[&1].clone();
        //     oneclone.remove(&segmentmap[&'c']);
        //     segmentmap.insert('f', *oneclone.iter().next().unwrap()); 
        // }
        {
            let (c, e) = (segmentmap[&'c'], segmentmap[&'e']);
            twothreefive.into_iter().for_each(|dg| {
                if dg.contains(&c) && dg.contains(&e) {
                    digitmap.insert(2, dg);
                } else if dg.contains(&c) && !dg.contains(&e) {
                    digitmap.insert(3, dg);
                } else {
                    digitmap.insert(5, dg);
                }
            });
        }
        // segmentmap.insert('b', *digitmap[&9].difference(&digitmap[&3]).next().unwrap());
        // {
        //     let mut min94:HashSet<&char> = digitmap[&9].difference(&digitmap[&4]).collect();
        //     min94.remove(segmentmap[&'a']);
        //     segmentmap.insert('g', min94.iter().next().unwrap());
        // }

        let mut res = String::new();
        output.iter().map(|s| s.chars().collect::<HashSet<char>>()).for_each(|m| {
            for (n, d) in &digitmap {
                if d.symmetric_difference(&m).count() == 0 {
                    res.push_str(&n.to_string()); break;
                }
            }
        });
        // println!("res {}", res);
        sum += res.parse::<isize>().unwrap();
    }
    
    println!("{}", sum);
}