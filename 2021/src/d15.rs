use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::VecDeque;

fn main() {
    let mut perigrid:Vec<Vec<usize>> = Vec::new();

    for l in BufReader::new(File::open("inputs/i15.txt").unwrap()).lines().map(|l| l.unwrap()) {
        let start:Vec<usize> = l.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
        let mut row = start.clone();
        for n in 1..5 {
            row.append(&mut start.iter().map(|d| ((d+n-1)%9)+1).collect());
        }
        perigrid.push(row);
    }
    {
        let ylen =  perigrid.len();
        for n in 1..5 {
            for i in 0..ylen {
                perigrid.push(perigrid[i].iter().map(|d| ((d+n-1)%9)+1).collect());
            }
        }
    }

    let (xmax, ymax) = (perigrid.len()-1 as usize, perigrid[0].len()-1 as usize);
    let mut distancegrid = vec![vec![usize::MAX; ymax+1]; xmax+1];
    distancegrid[0][0] = 0;

    let mut queue:VecDeque<(usize, usize)> = [(0,0)].into();
    while !queue.is_empty() {
        let (x, y) = queue.pop_back().unwrap();
        for (x1, y1) in neighbors(x, y, xmax, ymax) {
            let newdistance = distancegrid[x][y] + perigrid[x1][y1];
            if newdistance < distancegrid[x1][y1] {
                distancegrid[x1][y1] = newdistance;
                queue.push_front((x1, y1));
            }
        }
    }

    println!("{}", distancegrid[xmax][ymax]);
}

fn neighbors(x:usize, y:usize, xmax:usize, ymax:usize) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    if x > 0 { neighbors.push((x-1, y)); }
    if x < xmax { neighbors.push((x+1, y)); }
    if y > 0 { neighbors.push((x, y-1)); }
    if y < ymax { neighbors.push((x, y+1)); }
    neighbors
}

