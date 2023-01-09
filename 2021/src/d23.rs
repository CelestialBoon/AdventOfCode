use rustc_hash::{FxHashMap, FxHashSet};

fn main() {
    // functions for moving out of the room, and into rooms, and directly from room to room
    // then crabs move out of rooms if there is no alternative, trying always first to go into their own rooms, then all crabs in the hallway try to get back into the rooms
    // can go into own room: the way has to be clear, and there has to be only same crabs in the destination room
    //there can be a map of the spots to numbers, and a map of the numbers to spots, so we can use number ranges to move
    //and then ofc adding a cost for entering/leaving rooms, based on len()
    //order is: crabs in rooms try go go into rooms; crabs in hallways try to go into rooms; crabs in rooms try to go into hallways

    let initialstate = State {
        hallway:[5;11],
        rooms: [[1,3,3,2,], 
                [0,1,2,3,], 
                [1,0,1,3,], 
                [2,2,0,0,]],
    };

    let validhallways:[usize;7] = [0,1,3,5,7,9,10];
    
    let mut ins:FxHashMap<usize, FxHashSet<State>> = FxHashMap::default();
    let mut firsths = FxHashSet::default();
    firsths.insert(initialstate);
    ins.insert(0, firsths);
    let mut cost = 0usize;
    
    loop {
        if !ins.contains_key(&cost) || ins[&cost].is_empty() {
            cost +=1;
            if cost == 1000000 {break;}
        } else {
            let fs = pop(ins.entry(cost).or_insert(FxHashSet::default()));
            // if !fs.iseverythingalright() {
            //     panic!("not alright:{:?}", fs);
            // }
            if fs.isfinished() {
                println!("cost: {}", cost);
                break;
            }
            let mut newstates:Vec<_> = Vec::new();
            for i in 0..4 { //room to hallway
                if !fs.isempty(i) && !fs.contains(i) {
                    for h in validhallways.iter() {
                        match fs.roomtohallway(i, *h) {
                            None => (),
                            Some(d) => {
                                let mut s = fs.clone();
                                let (spot, crab) = s.first(i);
                                let newcost = cost + dcost(d, crab);
                                s.rooms[i][spot] = 5;
                                if s.hallway[*h] != 5 {panic!("aaaa hallway occupied")}
                                s.hallway[*h] = crab;
                                newstates.push((newcost, s));
                            }
                        }
                    }
                }
            }
            for h in validhallways.iter() { //hallway to room
                if fs.hallway[*h] < 5 {
                    match fs.hallwaytoroom(*h) {
                        None => (),
                        Some(d) => {
                            let mut s = fs.clone();
                            let crab = fs.hallway[*h];
                            let cu = crab as usize;
                            let newcost = cost + dcost(d, crab);
                            s.hallway[*h] = 5;
                            s.rooms[cu][s.len(cu)] = crab;
                            newstates.push((newcost, s));
                        }
                    }    
                }
            }
            for (c,s) in newstates {
                ins.entry(c).or_insert(FxHashSet::default()).insert(s);
            }
        }
    }
}

pub fn pop<T>(set: &mut FxHashSet<T>) -> T
where
    T: Eq + Clone + std::hash::Hash,
{
    let elt = set.iter().next().cloned().unwrap();
    set.remove(&elt);
    elt
}

fn range(a:usize, b:usize) -> std::ops::RangeInclusive<usize>  {
    if a < b {a..=b} else {b..=a}
}

fn hallwaydict(roomindex:usize) -> usize {roomindex*2+2}

fn dcost(distance:usize, crabtype:u8) -> usize {
    distance * match crabtype {
        0 => 1,
        1 => 10,
        2 => 100,
        3 => 1000,
        _ => panic!("aaa")
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    hallway:[u8;11],
    rooms:[[u8;4];4],
}
impl State {
    // fn iseverythingalright(&self) -> bool {
    //     let a: Vec<_> = self.rooms.iter().map(|r| r.iter()).flatten().chain(self.hallway.iter()).collect();
    //     a.iter().filter(|c| ***c == 0).count() == 4 && 
    //     a.iter().filter(|c| ***c == 1).count() == 4 && 
    //     a.iter().filter(|c| ***c == 2).count() == 4 && 
    //     a.iter().filter(|c| ***c == 3).count() == 4 
    // }
    fn isfinished(&self) -> bool {
        self.hallway.iter().all(|c| *c == 5) && 
        self.rooms.iter().enumerate().all(|(i, r)| r.iter().all(|c| *c == i as u8))
    }
    fn isempty(&self, roomi:usize) -> bool {
        self.len(roomi) == 0
    }
    fn first(&self, roomi:usize) -> (usize, u8) {
        let i = self.len(roomi) - 1;
        (i, self.rooms[roomi][i])
    }
    fn len(&self, roomi:usize) -> usize {
        self.rooms[roomi].iter().filter(|c| **c < 5).count()
    }
    fn contains(&self, roomi:usize) -> bool {
        let crab = roomi as u8;
        self.rooms[roomi].iter().all(|c| *c == crab || *c == 5)
    }
    
    fn roomtohallway(&self, startingroom:usize, endhall:usize) -> Option<usize> {
        let (hall1, hall2) = (hallwaydict(startingroom), endhall);
        range(hall1, hall2).all(|i| self.hallway[i] == 5).then(||
            i32::abs(hall1 as i32 - hall2 as i32) as usize + 5 - self.len(startingroom))
    }
    
    fn hallwaytoroom(&self, startinghall:usize) -> Option<usize> {
        let crab = self.hallway[startinghall];
        let cu = crab as usize;
        let hall2 = hallwaydict(cu);
        let hall1 = if startinghall < hall2 { startinghall+1 } else { startinghall-1 };
        (range(hall1, hall2).all(|i| self.hallway[i] == 5) && self.contains(cu)).then(||
            i32::abs(hall1 as i32 - hall2 as i32) as usize + 5 - self.len(cu))
    }
}