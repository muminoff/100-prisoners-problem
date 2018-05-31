extern crate rand;

use rand::{thread_rng, Rng};
use std::collections::{HashMap, HashSet};
use std::fmt;

const MAX: u8 = 10;
const LIMIT: u8 = MAX / 2;

#[derive(Debug)]
struct Prisoner {
    id: u8,
    drawers: HashSet<u8>,
}

impl Prisoner {
    fn found(&self) -> bool {
        self.drawers.contains(&self.id)
    }
}

fn main() {
    let mut drawers: Vec<u8> = (1..MAX + 1).collect();
    let mut prisoners: Vec<Prisoner> = Vec::new();
    for x in 1..MAX + 1 {
        let prisoner = Prisoner {
            id: x,
            drawers: HashSet::new(),
        };
        prisoners.push(prisoner);
    }
    let mut rng = thread_rng();
    rng.shuffle(&mut drawers);
    rng.shuffle(&mut prisoners);
    println!("Prisoners: {:?}", prisoners);
    println!("Drawers: {:?}", drawers);
    for prisoner in &mut prisoners {
        open_drawers(prisoner, &drawers);
        println!("{} > {:?} {}", prisoner.id, prisoner.drawers, prisoner.found());
    }
}

fn open_drawers(prisoner: &mut Prisoner, drawers: &Vec<u8>) {
    let mut index: usize = prisoner.id as usize - 1;
    for _ in 1..LIMIT + 1 {
        let drawer = drawers[index];
        prisoner.drawers.insert(drawer);
        index = drawer as usize - 1;
        if prisoner.id == drawer {
            break;
        }
    }
}
