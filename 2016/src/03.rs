use std::fs::File;
use std::io::{prelude::*, BufReader};
use itertools::Itertools;

fn main() {
    let file = File::open("inputs/i03.txt").unwrap();
    let reader = BufReader::new(file);

    let mut validnum = 0isize; 
    
    let mut trs = [[0i32; 3]; 3];
    for chunk in &reader.lines().chunks(3) {
        chunk.enumerate().for_each(|(i, s)| s.unwrap().split_whitespace().enumerate()
        .for_each(|(j, l)| trs[j][i] = l.parse::<i32>().unwrap()));
        trs.iter().for_each(|tr| if tr[0] < tr[1] + tr[2] && tr[1] < tr[0] + tr[2] && tr[2] < tr[0] + tr[1] {
            validnum +=1;});
            
        }
        
    // for l in BufReader::new(file).lines() {
    //     let tr:Vec<i32> = l.unwrap().split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
    //     if tr[0] < tr[1] + tr[2] && tr[1] < tr[0] + tr[2] && tr[2] < tr[0] + tr[1] {
    //         validnum +=1;
    //     }    
    // }
    println!("{}", validnum);
}
