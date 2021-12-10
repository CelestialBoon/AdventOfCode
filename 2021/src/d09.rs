use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let file = File::open("inputs/i09.txt").unwrap();
    let mut grid:Vec<Vec<usize>> = Vec::new();

    for l in BufReader::new(file).lines() {
        let line = l.unwrap();
        grid.push(line.chars().filter_map(|c| c.to_digit(10)).map(|d| d as usize).collect());
    }

    let xmax = grid.len() -1;
    let ymax = grid[0].len() -1;
    
    struct Rec<'s> { f: &'s dyn Fn(&Rec, usize, usize) -> () }
    let basin = |xlow, ylow| {
        let basin = Rc::new(RefCell::new(HashSet::<(usize, usize)>::new()));
        let rec = Rec {
            f: &|rec, x, y| {
                //search all neighbors
                for (i, j) in neighbors(x, y, xmax, ymax).into_iter()
                //if they are in the set or 9, abort
                .filter(|(i, j)| !basin.borrow().contains(&(*i,*j)) && grid[*i][*j] != 9) {
                //else search those places too recursively 
                    basin.borrow_mut().insert((i, j)); 
                    (rec.f)(&rec, i, j);
                }
            }
        };
        //start search here with the low point
        (rec.f)(&rec, xlow, ylow);
        basin
    };
    
    let mut basins:Vec<Rc<RefCell<HashSet<(usize, usize)>>>> = Vec::new();
    
    let mut sum = 0;
    for i in 0..=xmax {
        for j in 0..=ymax {
            let val = grid[i][j];
            if neighbors(i, j, xmax, ymax).into_iter().all(|(x, y)| grid[x][y]>val) {
                basins.push(basin(i, j));
                sum+=val+1; 
            }
        }
    };
        
    basins.sort_by(|b1, b2| b2.borrow().len().cmp(&b1.borrow().len()));
    println!("{} - {}", sum, basins.iter().take(3).fold(1, |acc, basin| { acc*basin.borrow().len() }));
}

fn neighbors(x:usize, y:usize, xmax:usize, ymax:usize) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    if x > 0 { neighbors.push(x-1, y); }
    if x < xmax { neighbors.push(x+1, y); }
    if y > 0 { neighbors.push(x, y-1); }
    if y < ymax { neighbors.push(x, y+1); }
    neighbors
}