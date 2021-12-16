use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::{HashSet, HashMap};
use regex::Regex;

fn main() {
    let file = File::open("inputs/i12.txt").unwrap();
    
    let mut map:HashMap<String, Vec<String>> = HashMap::new();
    let mut list:HashSet<String> = HashSet::new();

    let mut paths = 0;
    
    for l in BufReader::new(file).lines() {
        let c = l.unwrap().split('-').map(|s| s.to_string()).collect::<Vec<_>>();
        list.insert(c[0].clone()); list.insert(c[1].clone()); 
        map.entry(c[0].clone()).or_insert(Vec::new()).push(c[1].clone());
        map.entry(c[1].clone()).or_insert(Vec::new()).push(c[0].clone());
    }
    let mut visited:HashMap<String, bool> = HashMap::new();
    for s in list {
        visited.insert(s.to_string(), false);
    }
    visited.insert("start".to_string(), true);
    explore("start".to_string(), None, visited, &map, &mut paths);
    
    println!("{}", paths);
}

fn explore (loc:String, double:Option<String>, visited:HashMap<String, bool>, map:&HashMap<String, Vec<String>>, paths:&mut isize) {
    let up = Regex::new("^[A-Z]+$").unwrap();
    map[&loc].iter().filter(|l| {
        !visited[*l] || up.is_match(l) || (*l != "start" && double.is_none())
    }).for_each(|l| {
        if l == "end" {
            *paths+=1;
        } else {
            let newdouble = if double.is_some() { double.clone() }
            else if visited[l] && !up.is_match(l) { Some(l.to_string()) }
            else {None};
            let mut newvisited = visited.clone();
            newvisited.insert(l.to_string(), true);
            explore(l.to_string(), newdouble, newvisited, map, paths);
        }
    });

}
