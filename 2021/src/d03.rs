use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::VecDeque;

fn main() {
    let reader = BufReader::new(File::open("inputs/i03.txt").unwrap());

    let mut size = 0;
    let mut char_list : Vec<Vec<char>> = Vec::new();
    
    //parsing
    for line in reader.lines() {
        let l = line.unwrap();
        if size == 0 { size = l.len(); }
        char_list.push(l.chars().collect::<Vec<_>>());
    }

    fn zeros_ones(list: &Vec<Vec<char>>, position: usize) -> (i32, i32) {
        let (mut zeros, mut ones) = (0, 0);
        for n in 0..list.len() {
            match list[n][position] {
                '0' => zeros += 1,
                '1' => ones += 1,
                _ => { panic!("not zero or one"); }
            }
        }
        (zeros, ones)
    }
    
    //decoding binary
    fn to_number(vec : &Vec<char>) -> i32 {
        let st = vec.iter().collect::<String>();
        i32::from_str_radix(&st, 2).unwrap()
    }
    
    //gammaepsilon
    {
        let mut vgamma : VecDeque<char> = VecDeque::new();
        let mut vepsilon : VecDeque<char> = VecDeque::new();

        //double iteration
        for i in (0..size).rev() {
            let (zeros, ones) = zeros_ones(&char_list, i);
            if zeros > ones {
                vgamma.push_front('0');
                vepsilon.push_front('1');
            } else {
                vgamma.push_front('1');
                vepsilon.push_front('0');
            }
        }

        let gamma = to_number(&vgamma.into_iter().collect());
        let epsilon = to_number(&vepsilon.into_iter().collect());
        
        println!("gamma*epsilon = {}", gamma*epsilon);
    }
    //O2 CO2
    {
        enum Rating { Oxygen, Co2 }

        fn scrematura(olist : &Vec<Vec<char>>, length: usize, rating: Rating) -> Vec<char> {
            let mut list = olist.clone();
            for i in 0..length {
                let (zeros, ones) = zeros_ones(&list, i);
                let criteria = match &rating {
                    Rating::Oxygen => {if ones >= zeros {'1'} else {'0'}}
                    Rating::Co2 => {if ones >= zeros {'0'} else {'1'}}
                };
                list = list.into_iter().filter(|l| l[i] == criteria).collect();
                if list.len() == 1 { break; }
            }
            list[0].clone()
        }

        let vo2 = scrematura(&char_list, size, Rating::Oxygen);
        let vco2 = scrematura(&char_list, size, Rating::Co2);
        let o2 = to_number(&vo2);
        let co2 = to_number(&vco2);
        
        println!("o2*co2 = {}", o2*co2);
    }    
}

