use rand::{thread_rng};
use rand::seq::SliceRandom;

pub fn xy_to_idx(x: i32, y: i32) -> usize {
    (y * 80 + x) as usize
}

pub fn idx_to_xy(idx: i32) -> (i32, i32) {
    (idx % 80, idx / 80)
}

pub fn neighbours(idx: i32) -> Vec<usize> {
    let (x, y) = idx_to_xy(idx);

    let mut neighbours = Vec::new();

    if x + 1 < 80 {
        neighbours.push( xy_to_idx(x + 1, y ));
    }

    if x - 1 >= 0 {
        neighbours.push( xy_to_idx(x - 1, y ));
    }

    if y + 1 < 50 {
        neighbours.push(xy_to_idx(x, y + 1 ));
    }

    if y - 1 >= 0 {
        neighbours.push(xy_to_idx(x, y - 1));
    }

    neighbours.shuffle(&mut thread_rng());

    neighbours
}