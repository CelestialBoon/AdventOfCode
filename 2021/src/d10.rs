use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("inputs/i10.txt").unwrap();
    
    let mut sumdamaged = 0;
    let mut completescores:Vec<isize> = Vec::new();
    for l in BufReader::new(file).lines() {
        let mut completescore = 0;
        let mut levels:Vec<char> = Vec::new();
        for c in l.unwrap().chars() {
            if c == '(' || c == '[' || c == '{' || c == '<' {
                levels.push(c);
            } else {
                let res = {
                    let c1 = levels.pop().unwrap();
                    if c == match c1 {
                        '(' => ')',
                        '[' => ']',
                        '{' => '}',
                        '<' => '>',
                        _ => panic!("wrong opening character")
                    } { 0 } 
                    else { match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => panic!("wrong closing character")
                        }
                    } 
                };
                if res > 0 {
                    sumdamaged += res;
                    levels.clear();
                    break;
                }
            }
        }
        levels.iter().rev().for_each(|c| {
            completescore *=5;
            completescore += match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("wrong opening character")
            };
        });
        if completescore > 0 {
            completescores.push(completescore);
        }
    }
    completescores.sort();
    println!("{} - {}", sumdamaged, completescores[(completescores.len()-1) / 2]);
}

