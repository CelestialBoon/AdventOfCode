use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let reader = BufReader::new(File::open("inputs/i16.txt").unwrap());
    let rex = Regex::new(r"^Sue \d+: (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)$").unwrap();

    #[derive(Debug)]
    struct Sue {
        children: Option<isize>,
        cats: Option<isize>,
        samoyeds: Option<isize>,
        pomeranians: Option<isize>,
        akitas: Option<isize>,
        vizslas: Option<isize>,
        goldfish: Option<isize>,
        trees: Option<isize>,
        cars: Option<isize>,
        perfumes: Option<isize>,
    }

    impl Sue {
        pub fn new() -> Self {
            Self { children: None, cats: None, samoyeds: None, pomeranians: None, akitas: None, vizslas: None, goldfish: None, trees: None, cars: None, perfumes: None }
        }
    }

    let mut sues: Vec<Sue> = Vec::new();

    for line in reader.lines() {
        let l = line.unwrap();
        let cs = rex.captures(&l).unwrap();
        let mut sue = Sue::new();
        let mut map:HashMap<String, isize> = HashMap::new();
        
        map.insert(cs[1].to_string(), cs[2].parse::<isize>().unwrap());
        map.insert(cs[3].to_string(), cs[4].parse::<isize>().unwrap());
        map.insert(cs[5].to_string(), cs[6].parse::<isize>().unwrap());
        for (k, v) in map {
            match k.as_str() {
                "children" => sue.children = Some(v),
                "cats" => sue.cats = Some(v),
                "samoyeds" => sue.samoyeds = Some(v),
                "pomeranians" => sue.pomeranians = Some(v),
                "akitas" => sue.akitas = Some(v),
                "vizslas" => sue.vizslas = Some(v),
                "goldfish" => sue.goldfish = Some(v),
                "trees" => sue.trees = Some(v),
                "cars" => sue.cars = Some(v),
                "perfumes" => sue.perfumes = Some(v),
                _ => panic!("don't have match")
            }
        }
        sues.push(sue);
    }

    for (i, sue) in sues.iter().enumerate() {
        if sue.children.map_or(true, |a| a == 3) && sue.cats.map_or(true, |a| a > 7) && sue.samoyeds.map_or(true, |a| a == 2) && sue.pomeranians.map_or(true, |a| a < 3) && sue.akitas.map_or(true, |a| a == 0) && sue.vizslas.map_or(true, |a| a == 0) && sue.goldfish.map_or(true, |a| a < 5) && sue.trees.map_or(true, |a| a > 3) && sue.cars.map_or(true, |a| a == 2) && sue.perfumes.map_or(true, |a| a == 1) {
            println!("{}", i+1);
            break;
        }
    }
}