use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let mut grid:Vec<Vec<char>> = Vec::new();
    for l in BufReader::new(File::open("inputs/i25.txt").unwrap()).lines().map(|l| l.unwrap()) {
        grid.push(l.chars().collect());
    }

    let (my, mx) = (grid.len(), grid[0].len());

    let mut step = 0;
    loop {
        step+=1;
        let mut movingeast = Vec::new();
        for i in 0..my {
            for j in 0..mx {
                if grid[i][j] == '>' && grid[i][(j+1)%mx] == '.' {
                    movingeast.push((i, j));
                }
            }
        }
        for (i,j) in &movingeast {
            grid[*i][*j] = '.';
            grid[*i][(*j+1)%mx] = '>';
        }
        
        let mut movingsouth = Vec::new();
        for i in 0..my {
            for j in 0..mx {
                if grid[i][j] == 'v' && grid[(i+1)%my][j] == '.' {
                    movingsouth.push((i, j));
                }
            }
        }
        for (i,j) in &movingsouth {
            grid[*i][*j] = '.';
            grid[(*i+1)%my][*j] = 'v';
        }

        if movingsouth.len() + movingeast.len() == 0 {
            println!("{}", step);
            break;
        }
    }
}