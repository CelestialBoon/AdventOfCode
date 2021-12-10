use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("inputs/i02.txt").unwrap();
    let reader = BufReader::new(file);

    let nums = vec![vec![' ',' ','1',' ',' '], 
                    vec![' ','2','3','4',' '], 
                    vec!['5','6','7','8','9'], 
                    vec![' ','A','B','C',' '], 
                    vec![' ',' ','D',' ',' '], 
                    ];
    let mut code:String = String::new();

    let (mut x, mut y) = (1isize,1isize);
    for l in reader.lines() {
        l.unwrap().chars().for_each(|c| {
            match c {
                'U' => if x-y<2 && x+y>2 {y-=1},
                'D' => if y-x<2 && x+y<6 {y+=1},
                'L' => if y-x<2 && x+y>2 {x-=1},
                'R' => if x-y<2 && x+y<6 {x+=1},
                _ => panic!("wrong direction")
            }
        });
        code.push_str(&nums[y as usize][x as usize].to_string());
    }
    println!("{}", code);
}
