use std::fs::File;
use std::io::{prelude::*, BufReader};
// use std::collections::{HashSet, HashMap};
use regex::Regex;

fn main() {
    let file:Vec<String> = BufReader::new(File::open("inputs/i13.txt").unwrap()).lines().map(|l| l.unwrap()).collect();
    let rx1 = Regex::new(r"(^\d+),(\d+)$").unwrap();
    let rx2 = Regex::new(r"^fold along (x|y)=(\d+)$").unwrap();

    #[derive(Debug)]
    enum Folds { X, Y }

    let (mut firstx, mut firsty) = (0,0);
    let mut folds:Vec<Folds> = Vec::new();

    for line in file.iter() {
        if rx2.is_match(line) {
            let caps = rx2.captures(line).unwrap();
            if &caps[1] == "x" {
                folds.push(Folds::X);
                if firstx == 0 { firstx = caps[2].parse::<isize>().unwrap();}
            } else if &caps[1] == "y" {
                folds.push(Folds::Y);
                if firsty == 0 { firsty = caps[2].parse::<isize>().unwrap();}
            }
        }
    }

    let mut grid = vec![vec![false; (firstx*2+1) as usize]; (firsty*2+1) as usize];
    
    for line in file.iter() {
        match rx1.captures(line) {
            None => (),
            Some(caps) => {
                let (x, y) = (caps[1].parse::<usize>().unwrap(), caps[2].parse::<usize>().unwrap());
                grid[y][x] = true;
            }
        }
    }
    for f in folds {
        match f {
            Folds::X => {
                let half = (grid[0].len()-1)/2;
                for i in 0..grid.len() {
                    let row = &mut grid[i];
                    for j in 0..half {
                        let mirror = (half*2-j) as usize;
                        row[j] = row[mirror] || row[j];
                    }
                    row.resize(half, false);
                }
            },
            Folds::Y => {
                let half = ((grid.len()-1)/2) as usize;
                for i in 0..half {
                    let mirror = (grid.len()-i-1) as usize;
                    grid[i] = grid[i].iter().zip(grid[mirror].iter()).map(|(a, b)| *a || *b).collect();
                }
                grid.resize(half, Vec::new());
            }
        }
    }

    for row in grid {
        println!("{}", row.iter().map(|c| if *c {'#'} else {' '}).collect::<String>());
    }
}
