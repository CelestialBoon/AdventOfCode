fn main () {
    enum Move {
        MagicMissile,
        Drain,
        Shield,
        Poison,
        Recharge
    }
    let move_iter = vec![Move::MagicMissile, Move::Drain, Move::Shield, Move::Poison, Move::Recharge];

    struct State {
        hp:isize, 
        bosshp:isize, 
        mana:isize, 
        manaspent:isize,
        shield:isize, 
        poison:isize, 
        recharge:isize
    }
    impl State {
        pub fn copy(&self) -> Self {
            Self {hp:self.hp, bosshp:self.bosshp, mana:self.mana, manaspent:self.manaspent, shield:self.shield, poison:self.poison, recharge:self.recharge}
        }
    }

    fn turn(next_move:&Move, state:State, move_iter:&Vec<Move>) -> Option<isize> {
        let mut newstate = state.copy();
        let cost = match next_move {
            Move::MagicMissile => 53,
            Move::Drain => 73,
            Move::Shield => 113,
            Move::Poison => 173,
            Move::Recharge => 229
        };
        if state.bosshp <= 0 {
            return Some(state.manaspent);
        }

        if state.poison > 0 {
            newstate.bosshp -=3; newstate.poison -= 1;
        }
        if state.recharge > 0 {
            newstate.mana += 101; newstate.recharge -=1;
        }
        if state.shield > 0 {
            newstate.shield -=1;
        }
        
        if cost > newstate.mana || state.hp <= 0 {
            return None
        } else {
            newstate.mana -= cost;
            newstate.manaspent += cost;
        };

        match next_move {
            Move::MagicMissile => {newstate.bosshp -=4},
            Move::Drain => {newstate.bosshp -=2; newstate.hp +=2},
            Move::Shield => if newstate.shield == 0 {newstate.shield = 6} else {return None},
            Move::Poison => if newstate.poison == 0 {newstate.poison = 6} else {return None},
            Move::Recharge => if newstate.recharge == 0 {newstate.recharge = 5} else {return None},
        }
        //bossturn
        let mut armor = 0;
        if state.poison > 0 {
            newstate.bosshp -=3; newstate.poison -= 1;
        }
        if state.recharge > 0 {
            newstate.mana += 101; newstate.recharge -=1;
        }
        if state.shield > 0 {
            armor = 7; newstate.shield -=1;
        }
        newstate.hp -= 9 - armor;
        move_iter.iter().filter_map(|m| turn(m, newstate.copy(), &move_iter)).min()
    }

    let initialstate = State {hp:50, bosshp:58, mana:500, manaspent:0, shield:0, poison:0, recharge:0};

    println!("{}", move_iter.iter().filter_map(|m| turn(m, initialstate.copy(), &move_iter)).min().unwrap());
}