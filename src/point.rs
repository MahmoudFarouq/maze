
#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32
}
impl Point {
    pub fn new(tup: (i32, i32)) -> Self {
        Point {
            x: tup.0,
            y: tup.1,
        }
    }

}