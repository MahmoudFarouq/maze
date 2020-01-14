use rltk::{Rltk, Console, RGB};

use crate::point::*;
use crate::helpers::*;


pub struct Drawer;
impl Drawer {
    pub fn draw(&self, disjoint_set: &[i32], ctx : &mut Rltk) {
        for (child, parent) in disjoint_set.iter().enumerate() {
            // Render a tile depending upon the tile type
            match parent {
                -1 => {}
                _ => {

                    let mut from = Point::new(idx_to_xy(*parent));
                    let mut to = Point::new(idx_to_xy(child as i32));

                    Self::scale(&mut from, 2);
                    Self::scale(&mut to, 2);

                    let bridge = Self::get_bridge(&from, &to);

                    ctx.set(from.x  , from.y  , RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.), rltk::to_cp437('•'));
                    ctx.set(to.x    , to.y    , RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.), rltk::to_cp437('•'));
                    ctx.set(bridge.x, bridge.y, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.), rltk::to_cp437('•'));
                }
            }
        }
    }

    fn scale(point: &mut Point, factor: i32) {
        point.x *= factor;
        point.y *= factor;
    }

    fn get_bridge(from: &Point, to: &Point) -> Point {
        let result;

        match from.x == to.x {
            true => match from.y > to.y {
                true => result = Point{x: from.x, y: from.y - 1},
                false => result = Point{x: from.x, y: from.y + 1},
            },
            false => match from.x > to.x {
                true => result = Point{x: from.x - 1, y: from.y},
                false => result = Point{x: from.x + 1, y: from.y},
            },
        }

        result

    }

}
