use std::fs::File;
use std::io::{prelude::*, BufReader};
use regex::Regex;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let rx = Regex::new(r"\[|\]|\w+").unwrap();
    let mut nums:Vec<Vec<(isize, isize)>> = Vec::new();
    let mut num1:Vec<(isize, isize)> = Vec::new();

    for l in BufReader::new(File::open("inputs/i18.txt").unwrap()).lines().map(|l| l.unwrap()) {
        let mut lev = 0;
        let mut num:Vec<(isize, isize)> = Vec::new();
        for m in rx.find_iter(&l) {
            match m.as_str() {
                "[" => lev +=1,
                "]" => lev -=1,
                n => num.push((n.parse::<isize>().unwrap(), lev)),
            }
        }
        nums.push(num.clone());
        if num1.is_empty() { num1 = num; }
        else { num1 = addition(&num1, &num); }
    }

    let mut maxmag = 0;
    for i1 in 1..nums.len() {
        for i2 in 1..nums.len() {
            if i1 == i2 { continue; }
            let mag = magnitude(&addition(&nums[i1], &nums[i2]));
            if mag > maxmag {
                maxmag = mag;
            }
        }
    }
    println!("{} - {}", magnitude(&num1), maxmag);
}

fn addition(ns1:&Vec<(isize, isize)>, add:&Vec<(isize, isize)>) -> Vec<(isize, isize)> {
    let mut ns = ns1.clone();
    ns.append(&mut add.clone());
    ns.iter_mut().for_each(|(_, l)| {*l+=1;});
    loop {
        let expl = ns.iter().enumerate().find(|(_,(_,l))| *l==5);
        if expl.is_some() { //explosion
            let (i, &(n1, l)):(usize, &(isize, isize)) = expl.unwrap();
            let (n2, _):(isize, isize) = ns[i+1];
            if i>0 {
                ns[i-1].0 += n1;
            }
            if i<ns.len()-2 {
                ns[i+2].0 += n2;
            }
            ns[i] = (0, l-1);
            ns.remove(i+1);
            continue;
        }
        let spl = ns.iter().enumerate().find(|(_,(n,_))| *n>9 );
        if spl.is_some() { //split
            let (i, &(n, l)):(usize, &(isize, isize)) = spl.unwrap();
            ns[i] = (n/2, l+1);
            ns.insert(i+1, (if n%2==0 {n/2} else {n/2+1}, l+1));
            continue;
        }
        break;
    }
    ns
}

fn magnitude(ns1:&Vec<(isize, isize)>) -> isize {
    let ns: Rc<RefCell<Vec<(isize, isize)>>> = Rc::new(RefCell::new(ns1.clone()));
    let mag = |i, n1, n2, l| {
        let mut mns = ns.borrow_mut();
        mns[i] = (3*n1+2*n2, l-1);
        mns.remove(i+1);
    };

    'outer: loop {
        let cns = ns.borrow().clone();
        for li in (1..5).rev() {
            let r1 = cns.iter().enumerate().find(|(_,(_,l))| *l == li);
            if r1.is_some() {
                let (i, &(n1, l)):(usize, &(isize, isize)) = r1.unwrap();
                let (n2, _):(isize, isize) = cns[i+1];
                mag(i, n1, n2, l); continue 'outer;
            }
        }
        break;
    }
    let res = ns.borrow()[0].0; res
}