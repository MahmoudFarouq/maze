use rltk::{Rltk, GameState, Console};
use rand::{Rng};

use crate::state::*;
use crate::drawer::*;
use crate::helpers::*;


pub struct RandomizedKruskal {
    disjoint_set: Vec<i32>,
    drawer: Drawer
}
impl RandomizedKruskal {
    pub fn new(full_length: usize, drawer: Drawer) -> Self {
        RandomizedKruskal {
            disjoint_set: vec![-1; full_length],
            drawer
        }
    }

    fn find_set(&self, node_idx: usize) -> usize {
        let mut head: usize = node_idx;
        while self.disjoint_set[head] != -1 {
            // We can Improve this by Flattening, but this way we will lose track of what is connected to what
            head = self.disjoint_set[head] as usize;
        }
        head
    }

    fn connected(&self, left_idx: usize, right_idx: usize) -> bool {
        self.find_set(left_idx) == self.find_set(right_idx)
    }

    fn union(&mut self, left_idx: usize, right_idx: usize) {
        let left_parent = self.find_set(left_idx);
        let right_parent = self.find_set(right_idx);

        self.disjoint_set[left_parent] = right_parent as i32;
    }

    fn next_candidate_arc(&self) -> (usize, usize) {

        // number from 0 to 3999, and a 'random' one of his neighbors
        let from = rand::thread_rng().gen_range(0, 4000);
        let to = neighbours(from)[0];

        (from as usize, to as usize)
    }

}
impl State for RandomizedKruskal {
    fn update(&mut self) {

        let (from, to) = self.next_candidate_arc();

        if self.connected(from, to) {
            return;
        }

        self.union(from, to);
    }
}


impl GameState for RandomizedKruskal {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        self.update();
        self.drawer.draw(&self.disjoint_set, ctx);
    }
}