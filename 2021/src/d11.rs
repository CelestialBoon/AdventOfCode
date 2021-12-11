use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("inputs/i11.txt").unwrap();
    let mut grid = vec![vec![0usize; 10]; 10];
    let mut flashing = vec![vec![false; 10]; 10];
    //read input
    for (i, l) in BufReader::new(file).lines().enumerate() {
        grid[i] = l.unwrap().chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
    }

    let mut flashes = 0;
    let mut step = 0;
    loop {
        step +=1;
        //increase each spot
        grid.iter_mut().flat_map(|r| r.iter_mut()).for_each(|c| {*c+=1;});

        //increase neighbors of the flashing
        for i in 0..10 {
            for j in 0..10 {
                lighter(i, j, &mut grid, &mut flashing);
            }
        }
        //set 0 to all those that are > 9 and record the flashing
        grid.iter_mut().flat_map(|r| r.iter_mut()).for_each(|c| {
            if *c > 9 {
                *c = 0;
                if step <= 100 { flashes +=1; }
            }
        });
        
        if flashing.iter().flat_map(|r| r.iter()).all(|c| *c) {
            break;
        }
        flashing.iter_mut().flat_map(|r| r.iter_mut()).for_each(|c| { *c = false; });
    }
    
    println!("{} - {}", flashes, step);
}

fn lighter(i:usize, j:usize, mut grid:&mut Vec<Vec<usize>>, mut flashing:&mut Vec<Vec<bool>>) {
    if grid[i][j] > 9 && !flashing[i][j] {
        flashing[i][j] = true;
        neighbors(i, j).into_iter().for_each(|(i1, j1)| {
            grid[i1][j1]+=1;
            lighter(i1, j1, &mut grid, &mut flashing);
        })
    }
}

fn neighbors(x:usize, y:usize) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    for i in 0..=2 {
        for j in 0..=2 {
            if (i != 1 || j != 1) && i+x > 0 && i+x < 11 && j+y > 0 && j+y < 11 {
                neighbors.push((i+x-1, j+y-1));
            }
        }
    }
    neighbors
}