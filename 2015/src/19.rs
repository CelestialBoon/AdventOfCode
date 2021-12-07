use std::fs::File;
use std::io::{prelude::*, BufReader};
use regex::Regex;

fn main() {
    let reader = BufReader::new(File::open("inputs/i19.txt").unwrap());
    let goal = "CRnCaCaCaSiRnBPTiMgArSiRnSiRnMgArSiRnCaFArTiTiBSiThFYCaFArCaCaSiThCaPBSiThSiThCaCaPTiRnPBSiThRnFArArCaCaSiThCaSiThSiRnMgArCaPTiBPRnFArSiThCaSiRnFArBCaSiRnCaPRnFArPMgYCaFArCaPTiTiTiBPBSiThCaPTiBPBSiRnFArBPBSiRnCaFArBPRnSiRnFArRnSiRnBFArCaFArCaCaCaSiThSiThCaCaPBPTiTiRnFArCaPTiBSiAlArPBCaCaCaCaCaSiRnMgArCaSiThFArThCaSiThCaSiRnCaFYCaSiRnFYFArFArCaSiRnFYFArCaSiRnBPMgArSiThPRnFArCaSiRnFArTiRnSiRnFYFArCaSiRnBFArCaSiRnTiMgArSiThCaSiThCaFArPRnFArSiRnFArTiTiTiTiBCaCaSiRnCaCaFYFArSiThCaPTiBPTiBCaSiThSiRnMgArCaF";
    let mut replacements: Vec<(String, String)> = Vec::new();

    let rx = Regex::new(r"^(\w+) => (\w+)$").unwrap();
    for line in reader.lines() {
        let l = line.unwrap();
        let cap = rx.captures(&l).unwrap();
        replacements.push((cap[1].chars().rev().collect::<String>(), cap[2].chars().rev().collect::<String>()));
    }
    
    let mut steps = 0;
    let mut str1 = goal.chars().rev().collect::<String>();
    let mut str2;
    loop {
        for (a, b) in &replacements {
            str2 = str1.replacen(b, a, 1);
            if str2 != str1 {
                steps += 1;
                str1 = str2;
                // println!("{}", str1);
            }
        }
        if str1 == "FH" || str1 == "lAN" || str1 == "gMO" {
            println!("{}", steps +1);
            break;
        }
    }
}
