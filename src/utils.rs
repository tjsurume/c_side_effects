use crate::prelude::*;
// use rltk::Point;

#[derive(Component, Copy, Clone, PartialEq, Eq, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Position {
    pub fn new(x:i32, y:i32, z:i32) -> Self {
        Self{x, y, z}
    }
    pub fn new_from2d(x:i32, y:i32) -> Self {
        Self{x, y, z:0}
    }
}


impl From<Point> for Position {
    fn from(item: Point) -> Self {
        Position { x:item.x, y:item.y, z:0 }
    }
}

impl From<(Point, i32)> for Position {
    fn from ((point, new_z): (Point, i32)) -> Self {
        Position { x:point.x, y:point.y, z: new_z }
    }
}

impl From<Position> for Point {
    fn from(item: Position) -> Self {
        Point { x:item.x, y:item.y}
    }
}
