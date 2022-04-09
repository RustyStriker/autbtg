use std::{ops::{Add, Sub}, fmt::Display};


#[derive(Clone, Copy, Debug, Eq, Hash, PartialOrd, Ord)]
pub struct IVec2 {
    x: i32,
    y: i32,
}
impl IVec2 {
    pub const ZERO: IVec2 = IVec2 { x: 0, y: 0};
    
    pub fn new(x: i32, y: i32) -> IVec2 {
        IVec2 { x, y }
    }
    pub fn dot(self, rhs: Self) -> i32 {
        self.x * rhs.x + self.y * rhs.y
    }
    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
    pub fn to_cursor(self) -> termion::cursor::Goto {
        termion::cursor::Goto(self.x as u16, self.y as u16)
    }
    #[must_use]
    pub fn min(self, other: IVec2) -> IVec2 {
        IVec2 { 
            x: self.x.min(other.x), 
            y: self.y.min(other.y)
        }
    }
}
impl Add for IVec2 {
    type Output = IVec2;

    fn add(self, rhs: Self) -> Self::Output {
        IVec2 { 
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub for IVec2 {
    type Output = IVec2;

    fn sub(self, rhs: Self) -> Self::Output {
        IVec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Display for IVec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl PartialEq for IVec2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
