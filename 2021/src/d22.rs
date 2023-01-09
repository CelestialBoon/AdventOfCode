use std::fs::File;
use std::io::{prelude::*, BufReader};
use regex::Regex;
use std::cmp::{min, max};

fn main() {
    let mut inputs:Vec<Cuboid> = Vec::new();
    let mut map:Vec<Cuboid> = Vec::new();

    let rx = Regex::new(r"^(on|off) x=(-?\w+)\.\.(-?\w+),y=(-?\w+)\.\.(-?\w+),z=(-?\w+)\.\.(-?\w+)$").unwrap();
    for l in BufReader::new(File::open("inputs/i21.txt").unwrap()).lines().map(|l| l.unwrap()) {
        let caps = rx.captures(&l).unwrap();
        let onoff = caps[1] == *"on";
        let ns:Vec<_> = (2..8).map(|n| caps[n].parse::<i64>().unwrap()).collect();
        let rect = Cuboid::new(onoff, ns[0], ns[1], ns[2], ns[3], ns[4], ns[5]).unwrap();
        inputs.push(rect);
    }
    for mut c in inputs.into_iter().rev() {
        let mut pieces1 = vec![c.clone()];
        let mut pieces2 = Vec::new();
        for c1 in &map {
            for p1 in &pieces1 {
                pieces2.append(&mut p1.subtract(c1));
            }
            pieces1 = pieces2;
            pieces2 = Vec::new();
        }
        c.volume = pieces1.iter().fold(0, |acc,c| acc + c.volume);
        map.push(c);
    }

    println!("{}", map.iter().filter(|c| c.onoff).fold(0, |acc, c| acc + c.volume));
}

#[derive(PartialEq, Eq, Clone)]
struct Cuboid {
    onoff:bool,
    x1:i64,
    x2:i64,
    y1:i64,
    y2:i64,
    z1:i64,
    z2:i64,
    volume:i64
}
impl Cuboid {
    fn new(onoff:bool, x1:i64, x2:i64, y1:i64, y2:i64, z1:i64, z2:i64) -> Option<Cuboid> {
        if x1 > x2 || y1 > y2 || z1 > z2 {
            None
        } else {
            Some( Cuboid { onoff:onoff, x1:x1, x2:x2, y1:y1, y2:y2, z1:z1, z2:z2, volume:(x2-x1+1)*(y2-y1+1)*(z2-z1+1)})
        }
    }

    fn intersect(&self, other:&Cuboid) -> Option<Cuboid> {
            Cuboid::new(self.onoff, 
                max(self.x1, other.x1), min(self.x2, other.x2), 
                max(self.y1, other.y1), min(self.y2, other.y2),
                max(self.z1, other.z1), min(self.z2, other.z2))
    }
    fn subtract(&self, other:&Cuboid) -> Vec<Cuboid> {
        match self.intersect(other) {
            None => vec![self.clone()],
            Some(a) if a == *self => vec![],
            _ => vec![
                Cuboid::new(self.onoff, self.x1, other.x1-1, self.y1, self.y2, self.z1, self.z2),
                Cuboid::new(self.onoff, other.x2+1, self.x2, self.y1, self.y2, self.z1, self.z2),
                Cuboid::new(self.onoff, other.x1, other.x2, self.y1, other.y1-1, self.z1, self.z2),
                Cuboid::new(self.onoff, other.x1, other.x2, other.y2+1, self.y2, self.z1, self.z2),
                Cuboid::new(self.onoff, other.x1, other.x2, other.y1, other.y2, self.z1, other.z1-1),
                Cuboid::new(self.onoff, other.x1, other.x2, other.y1, other.y2, other.z2+1, self.z2),
                ].into_iter().filter_map(|c| c.and_then(|c1| c1.intersect(self))).collect(),
            }
    }
}