// https://en.wikipedia.org/wiki/Maze_generation_algorithm

use rltk::{Rltk};

mod point;
mod tile;
mod state;
mod helpers;
mod drawer;
mod dfs;
mod rand_kruskal;
mod rand_prim;

use crate::drawer::*;
use crate::dfs::*;
use crate::rand_kruskal::*;
use crate::rand_prim::*;


fn main() {
    let context = Rltk::init_simple8x8(80 * 2, 50 * 2, "Hello Rust World", "resources");

    let drawer = Drawer;

    let gs = RandomizedPrim::new(50 * 80, 25*40, drawer);
    // let gs = RandomizedKruskal::new(50 * 80, drawer);
    // let gs = DepthFirst::new(50 * 80, 25*40, drawer);

    rltk::main_loop(context, gs);
}