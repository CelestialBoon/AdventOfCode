use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::{HashSet, HashMap};

type Point=(isize, isize, isize);

fn main() {
    let mut beacons:Vec<Vec<Point>> = Vec::new();
    let mut scanner:Vec<Point> = Vec::new();
    for l in BufReader::new(File::open("inputs/i19.txt").unwrap()).lines().map(|l| l.unwrap()) {
        if l.contains("---") {
        } else if l.is_empty() {
            beacons.push(scanner);
            scanner = Vec::new();
        } else {
            let coords:Vec<isize> = l.split(',').map(|n| n.parse::<isize>().unwrap()).collect();
            scanner.push((coords[0], coords[1], coords[2]));
        }
    }
    beacons.push(scanner);
    let sclen = beacons.len();
    
    let mut absdistances:Vec<HashSet<Point>> = Vec::new();
    let mut alldists:Vec<Vec<HashSet<Point>>> = Vec::new();
    let mut alldistsord:Vec<Vec<Vec<(Point, usize, usize)>>> = Vec::new();
    for s in &beacons {
        let mut ds:HashSet<Point> = HashSet::new();
        for i in 0..s.len() {
            for j in 0..i {
                ds.insert(distabs(&s[i], &s[j]));
            }
        }
        absdistances.push(ds);
        let mut dists:Vec<HashSet<Point>> = Vec::new();
        let mut distsord:Vec<Vec<(Point, usize, usize)>> = Vec::new();
        for or in 0..24 {
            let mut set:HashSet<Point> = HashSet::new();
            let mut distord:Vec<(Point, usize, usize)> = Vec::new();
            let os:Vec<Point> = s.iter().map(|p| oriented(p, or)).collect();
            for i in 0..os.len() {
                for j in 0..i {
                    set.insert(diff(&os[i], &os[j]));
                    distord.push((diff(&os[i], &os[j]), i, j));
                }
            }
            dists.push(set);
            distsord.push(distord);
        }
        alldists.push(dists);
        alldistsord.push(distsord);
    }
    
    let mut oriensmap:HashMap<usize, Vec<(usize, Point, usize)>> = HashMap::new();
    for i in 0..sclen {
        'search: for j in 0..sclen {
            if i == j { continue; }
            let c = absdistances[i].intersection(&absdistances[j]).count();
            if c >= 66 { //we have to align i to j
                let dis = &alldists[i];
                let dj = &alldists[j][0];
                for o in 0..24 {
                    let oc = dis[o].intersection(&dj).count();
                    if oc > 10 {
                        let dio = &alldistsord[i][o];
                        let djo = &alldistsord[j][0];
                        //figure out the origin: take two points which have the same dist
                        for &(id, ip1, _ip2) in dio {
                            for &(jd, jp1, _jp2) in djo {
                                if id == jd {
                                    let diff = diff(&beacons[j][jp1], &oriented(&beacons[i][ip1], o));
                                    oriensmap.entry(j).or_insert(Vec::new()).push((i, diff, o));
                                    continue 'search;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    let mut done:HashSet<usize> = HashSet::new();
    let (beaconspos, scannerspos) = chain(0, &mut done, &oriensmap, &beacons);

    let mut maxdist = 0;
    for p1 in &scannerspos {
        for p2 in &scannerspos {
            let dist = isize::abs(p1.0-p2.0) + isize::abs(p1.1-p2.1) + isize::abs(p1.2-p2.2);
            if dist>maxdist {
                maxdist = dist;
            }
        }
    }
    println!("{} - {}", beaconspos.len(), maxdist);
}         
    
fn chain(n:usize, done:&mut HashSet<usize>, oriensmap:&HashMap<usize, Vec<(usize, Point, usize)>>, beacons:&Vec<Vec<Point>>) -> (HashSet<Point>, Vec<Point>) {
    let mut points:HashSet<Point> = beacons[n].clone().into_iter().collect();
    let mut origins:Vec<Point> = vec![(0,0,0)];
    done.insert(n);
    for (n1, origin1, o1) in &oriensmap[&n] {
        if !done.contains(&n1) {
            let (mut newpoints, mut neworigins):(HashSet<_>, Vec<_>) = chain(*n1, done, oriensmap, beacons);
            newpoints = newpoints.into_iter().map(|p| sum(origin1, &oriented(&p, *o1))).collect();
            points = points.union(&newpoints).map(|&p| p).collect();
            neworigins = neworigins.into_iter().map(|p| sum(origin1, &oriented(&p, *o1))).collect();
            origins.append(&mut neworigins);
        }
    }
    (points, origins)
}

fn distabs(a:&Point, b:&Point) -> Point {
    let mut v:Vec<isize> = (vec![a.0-b.0, a.1-b.1, a.2-b.2]).into_iter().map(|n| isize::abs(n)).collect();
    v.sort();
    (v[0], v[1], v[2])
}

fn sum(a:&Point, b:&Point) -> Point {
    (a.0+b.0, a.1+b.1, a.2+b.2)
}

fn diff(a:&Point, b:&Point) -> Point {
    (a.0-b.0, a.1-b.1, a.2-b.2)
}

fn oriented(rp:&Point, orien:usize) -> Point {
    let &(x, y, z) = rp;
    match orien {
        0 => (x, y, z),
        1 => (-x, -y, z),
        2 => (-y, x, z),
        3 => (y, -x, z),
        4 => (y, x, -z),
        5 => (-y, -x, -z),
        6 => (x, -y, -z),
        7 => (-x, y, -z),
        8 => (z, y, -x),
        9 => (-z, -y, -x),
        10 => (y, -z, -x),
        11 => (-y, z, -x),
        12 => (y, z, x),
        13 => (-y, -z, x),
        14 => (z, -y, x),
        15 => (-z, y, x),
        16 => (x, z, -y),
        17 => (-x, -z, -y),
        18 => (z, -x, -y),
        19 => (-z, x, -y),
        20 => (z, x, y),
        21 => (-z, -x, y),
        22 => (x, -z, y),
        23 => (-x, z, y),
        _ => panic!("wrong orien")
    }
}