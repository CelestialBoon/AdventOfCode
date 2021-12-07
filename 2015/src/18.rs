use std::fs::File;
use std::io::{prelude::*, BufReader};


fn main() {
    let mut grid = vec![vec!['a';100]; 100];
    // let mut grid2 = grid.clone();
    let reader = BufReader::new(File::open("inputs/i18.txt").unwrap());
    
    for (i, l) in reader.lines().enumerate() {
        let line = l.unwrap();
        grid[i] = line.chars().collect();
    }

    for _ in 0..100 {
        grid = step(&grid);
    }

    println!("{}", grid.iter().map(|row| row.iter()).flatten().filter(|p| **p == '#').count());
}

fn step(grid:&Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut grid2 = grid.clone();
    for i in 0usize..100 {
        for j in 0usize..100 {
            let ons = neighbors(i, j).iter().filter(|(a, b)| grid[*a][*b] == '#').count();
            grid2[i][j] = match grid[i][j] {
                '#' => if ons == 2 || ons == 3 {'#'} else {'.'},
                '.' => if ons == 3 {'#'} else {'.'},
                _ => panic!("anomalous board")
            }
        }
    }
    grid2[0][0] = '#';
    grid2[0][99] = '#';
    grid2[99][0] = '#';
    grid2[99][99] = '#';
    grid2
}

fn neighbors(x:usize, y:usize) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    match (x, y) {
        (0, 0) => {
            neighbors.push((1, 0));
            neighbors.push((1, 1));
            neighbors.push((0, 1));
        },
        (0, 99) => {
            neighbors.push((1, 99));
            neighbors.push((1, 98));
            neighbors.push((0, 98));
        },
        (99, 0) => {
            neighbors.push((99, 1));
            neighbors.push((98, 1));
            neighbors.push((98, 0));
        },
        (99, 99) => {
            neighbors.push((98, 99));
            neighbors.push((98, 98));
            neighbors.push((99, 98));
        },
        (0, b) => {
            neighbors.push((0, b-1));
            neighbors.push((0, b+1));
            neighbors.push((1, b-1));
            neighbors.push((1, b));
            neighbors.push((1, b+1));
        },
        (99, b) => {
            neighbors.push((99, b-1));
            neighbors.push((99, b+1));
            neighbors.push((98, b-1));
            neighbors.push((98, b));
            neighbors.push((98, b+1));
        },
        (a, 0) => {
            neighbors.push((a-1, 0));
            neighbors.push((a+1, 0));
            neighbors.push((a-1, 1));
            neighbors.push((a, 1));
            neighbors.push((a+1, 1));
        },
        (a, 99) => {
            neighbors.push((a-1, 99));
            neighbors.push((a+1, 99));
            neighbors.push((a-1, 98));
            neighbors.push((a, 98));
            neighbors.push((a+1, 98));
        },
        (a, b) => {
            neighbors.push((a-1, b-1));
            neighbors.push((a-1, b));
            neighbors.push((a-1, b+1));
            neighbors.push((a, b-1));
            neighbors.push((a, b+1));
            neighbors.push((a+1, b-1));
            neighbors.push((a+1, b));
            neighbors.push((a+1, b+1));
        }
    }
    neighbors
}