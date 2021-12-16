use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

fn main() {
    let file:Vec<String> = BufReader::new(File::open("inputs/i14.txt").unwrap()).lines().map(|l| l.unwrap()).collect();
    let chars:Vec<char> = file[0].chars().collect();

    let mut rules:HashMap<(char, char), char> = HashMap::new();
    file.iter().skip(2).for_each(|l| {
        let cs = l.chars().collect::<Vec<char>>();
        rules.insert((cs[0], cs[1]), cs[6]);
    });
    let mut ruleschain:HashMap<(char, char), ((char, char), (char, char))> = HashMap::new();
    for ((a, b), c) in &rules {
        ruleschain.insert((*a, *b), ((*a, *c), (*c, *b)));
    } 
    let mut rulescount1:HashMap<(char,char), isize> = HashMap::new();
    for i in 0..chars.len()-1 {
        *rulescount1.entry((chars[i], chars[i+1])).or_insert(0) +=1;
    }

    let mut rulescount2:HashMap<(char,char), isize>;
    for _ in 0..40 {
        rulescount2 = HashMap::new();
        for (k, n) in &rulescount1 {
            let (nr1, nr2) = ruleschain[k];
            *rulescount2.entry(nr1).or_insert(0) +=n; 
            *rulescount2.entry(nr2).or_insert(0) +=n; 
        }
        rulescount1 = rulescount2;
    }

    let mut tally:HashMap<char, isize> = HashMap::new();
    for ((a, _), n) in &rulescount1 {
        *tally.entry(*a).or_insert(0) +=n;
    }
    *tally.entry(chars[chars.len()-1]).or_insert(0) +=1;

    let (mut max, mut min) = (0, isize::MAX);
    for (_, &n) in &tally {
        if n>max { max = n;}
        if n<min { min = n;}
    }
    println!("{}", max - min);
}
