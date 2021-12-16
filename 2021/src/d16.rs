use std::fs::File;
use std::io::Read;

fn main() {
    let mut buffer = String::new();
    File::open("inputs/i16.txt").unwrap().read_to_string(&mut buffer).expect("read failed");
    let mut binstr:String = buffer.chars().map(to_binary).collect();
   
    let (op, _) = parse(&mut binstr);    
    let mut sum = 0;
    countversion(&op, &mut sum);
    println!("versions: {} - result: {}", sum, op.number);
}

fn parse(s:&mut String) -> (Operator, isize) {
    let mut len = 0;
    let version = split(s, 3, &mut len);
    let typeid = split(s, 3, &mut len);
    
    let mut op = Operator {
        version:version, typeid:typeid, lengthid:false, number:0, subpackets:Vec::new()
    };
    if typeid == 4 { //business as usual
        let mut snum = String::new(); 
        loop {
            let mut iter = s.drain(..5);
            len +=5;
            let control = iter.by_ref().next().unwrap();
            snum.push_str(&iter.collect::<String>());
            if control == '0' {break;}
        }
        op.number = isize::from_str_radix(&snum, 2).unwrap();
    } else {
        op.lengthid = split(s, 1, &mut len) == 1;
        if op.lengthid { //number of subpackets in 11 bits
            let numsubpackets = split(s, 11, &mut len);
            for _ in 0..numsubpackets {
                let (sop, slen) = parse(s);
                op.subpackets.push(sop);
                len += slen;
            }
        } else { //length of subpackets in 15 bits
            let lensubpackets = split(s, 15, &mut len);
            len += lensubpackets;
            let mut sumlen = 0;
            while sumlen < lensubpackets {
                let (sop, slen) = parse(s);
                op.subpackets.push(sop);
                sumlen += slen;
            }
        }
        
        op.number = match typeid {
            0 => op.subpackets.iter().fold(0, |acc, p| acc + p.number),
            1 => op.subpackets.iter().fold(1, |acc, p| acc * p.number),
            2 => op.subpackets.iter().fold(isize::MAX, |acc, p| if p.number < acc {p.number} else {acc}),
            3 => op.subpackets.iter().fold(0, |acc, p| if p.number > acc {p.number} else {acc}),
            5 => if op.subpackets[0].number > op.subpackets[1].number {1} else {0},
            6 => if op.subpackets[0].number < op.subpackets[1].number {1} else {0},
            7 => if op.subpackets[0].number == op.subpackets[1].number {1} else {0},
            _ => panic!("wrong typeid")
        };
    }

    (op, len)
}

fn countversion(op:&Operator, sum:&mut isize) {
    *sum += op.version;
    for op1 in &op.subpackets {
        countversion(&op1, sum);
    }
}

fn split(s:&mut String, n:usize, len:&mut isize) -> isize {
    *len += n as isize;
    isize::from_str_radix(&s.drain(..n).collect::<String>(), 2).unwrap()
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => panic!("wrong hexa"),
    }
}

#[derive(Debug)]
struct Operator {
    version:isize,
    typeid:isize,
    lengthid:bool,
    number:isize,
    subpackets:Vec<Operator>,
}