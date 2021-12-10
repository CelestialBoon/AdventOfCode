use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

//each number has reference to what boards
//and each board has 25 numbers with yes/no (dict?), 5 rows and 5 columns (10 arrays of numbers will do)
//then each number you go to the refrence, update the dict, check those rows and columns
//and when you get a hit on those you're done, you just check the dict for the unused numbers and bob's your uncle

fn main() {
    let reader = BufReader::new(File::open("inputs/i04.txt").unwrap());

    let mut iter = reader.lines();
    let mut b_firstline = true;
    let mut firstline: Vec<isize> = Vec::new();

    fn parse_numbers(s:String) -> Vec<isize> {
        s.split_whitespace().map(|n| n.parse::<isize>().unwrap()).collect()
    }

    let mut numtobingo: HashMap<isize, Vec<isize>> = HashMap::new();
    let mut bblist:Vec<BingoBoard> = Vec::new();
    
    //parsing
    let mut bb: BingoBoard;
    loop {
        match iter.next() {
            None => break,
            Some(Ok(s)) => {
                if b_firstline {
                    firstline = s.split(',').map(|n| n.parse::<isize>().unwrap()).collect();
                    b_firstline = false;
                } else {
                    let rows: Vec<Vec<isize>> = (0..5).map(|_| parse_numbers(iter.next().unwrap().unwrap())).collect();
                    let numbers: Vec<isize> = rows.iter().flatten().copied().collect();
                    let columns: Vec<Vec<isize>> = (0..5).map(|n1| (0..5).map(|n2| rows[n2][n1]).collect()).collect();
                    let mut bingo_map : HashMap<isize, bool> = HashMap::new();
                    for n in &numbers {
                        bingo_map.insert(*n, false);           
                    };
                    
                    bb = BingoBoard {
                        rows: rows,
                        columns: columns,
                        bingo_map: bingo_map,
                        won: false
                    };
                    //add all to some struct
                    bblist.push(bb);
                    let len = (bblist.len() as isize) - 1;
                    
                    //add the struct to the numtobingo
                    for n in &numbers {
                        numtobingo.entry(*n).or_insert(Vec::new()).push(len);
                    };
                }
            },
            _ => panic!("reading error")
        }
    }
    let numboards = bblist.len();
    let mut boardswon = 0;

    'outer: for n in firstline {
        for bbn in &numtobingo[&n] {
            bblist.get_mut(*bbn as usize).unwrap().bingo_map.insert(n, true);
        }

        for bbn in &numtobingo[&n] {
            let bb = &mut bblist[*bbn as usize];
            if !bb.won && bb.rows.iter().chain(bb.columns.iter()).any(|rc| rc.iter().all(|n| bb.bingo_map[n])) {
                boardswon += 1;
                bb.won = true;
                if boardswon == numboards || boardswon == 1 {
                    let unmarkedsum = bb.bingo_map.iter().filter_map(|(n, marked)| (!marked).then(|| *n)).fold(0, |acc, n| acc+n);
                    println!("{} * {} = {}", n, unmarkedsum, unmarkedsum*n);
                    if boardswon == numboards {break 'outer;}
                }
            }
        }
    }
}

struct BingoBoard {
    rows: Vec<Vec<isize>>,
    columns: Vec<Vec<isize>>,
    bingo_map: HashMap<isize, bool>,
    won: bool
}