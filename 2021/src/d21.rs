use std::cmp::{min, max};

fn main() {
    let (ipos1, ipos2) = (9, 10);
    
    let (mut score1, mut score2, mut dierolls) = (0,0,0);
    {
        let (mut pos1, mut pos2) = (ipos1, ipos2);
        while score1 < 1000 && score2 < 1000 {
            pos1 = (pos1 + 3*dierolls + 5)%10 + 1;
            dierolls +=3;
            score1 += pos1;
            if score1 >= 1000 {break;}
            pos2 = (pos2 + 3*dierolls + 5)%10 + 1;
            dierolls +=3;
            score2 += pos2;
        }
    }
    
    let probs = vec![(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
    let mut wins = [0,0];
    
    let mut turn = |state:State, player:usize| -> Vec<State> {
        probs.iter().filter_map(|(r,w)| {
            let newpos = (state.pos[player] + *r -1)%10 + 1;
            let newscore = state.score[player] + newpos;
            let newweight = state.weight * w;
            if newscore < 21 {
                let mut newstate = state.clone();
                newstate.pos[player] = newpos;
                newstate.score[player] = newscore;
                newstate.weight = newweight;
                Some(newstate)
            } else {
                wins[player] += newweight; None
            }
        }).collect()
    };

    let mut states1:Vec<State> = Vec::from([State{pos:[ipos1, ipos2], score:[0,0], weight:1}]);
    let mut states2:Vec<State> = Vec::new();
    while !states1.is_empty() {
        states2.append(&mut turn(states1.pop().unwrap(), 0));
        while !states2.is_empty() {
            states1.append(&mut turn(states2.pop().unwrap(), 1));
        }
    }

    println!("{} - {}", min(score1, score2)*dierolls, max(wins[0], wins[1]));
}

#[derive(Clone)]
struct State {
    pos:[isize;2],
    score:[isize;2],
    weight:isize,
}