use std::cmp::PartialEq;

#[derive(Copy, Clone, PartialEq)]
pub struct RailType(RailShape, RailClass);

#[derive(Copy, Clone, PartialEq)]
pub enum RailShape {
    Empty,
    Horizontal,
    Vertical,
    NorthEast,
    SouthEast,
    NorthWest,
    SouthWest,
    TNorth,
    TEast,
    TSouth,
    TWest,
    Cross,
}

#[derive(Copy, Clone)]
pub enum RailClass {
    Normal,
    Highspeed,
}

impl RailType {
    pub fn new(shape: RailShape, class: RailClass) -> RailType {
        RailType(shape, class)
    }

    pub fn shape(&self) -> RailShape {
        self.0
    }

    pub fn class(&self) -> RailClass {
        self.1
    }
    
    /// Returns a list of boolean values indicating whether the rail 
    /// can connect to other rails. The order is as follows:
    /// 1) Can connect to north
    /// 2) Can connect to east
    /// 3) Can connect to south
    /// 4) Can connect to west
    pub fn connections(&self) -> [bool; 4] {
        match self.shape() {
            RailShape::Empty => { [false, false, false, false] }
            RailShape::Horizontal => { [false, true, false, true] }
            RailShape::Vertical => { [true, false, true, false] }
            RailShape::NorthEast => { [true, true, false, false] }
            RailShape::SouthEast => { [false, true, true, false] }
            RailShape::NorthWest => { [true, false, false, true] }
            RailShape::SouthWest => { [false, false, true, true] }
            RailShape::TNorth => { [true, true, false, true] }
            RailShape::TEast => { [true, true, true, false] }
            RailShape::TSouth => { [false, true, true, true] }
            RailShape::TWest => { [true, false, true, true] }
            RailShape::Cross => { [true, true, true, true] }
        }
    }
}

impl Into<char> for RailType {
    fn into(self) -> char {
        if self.shape() == RailShape::Empty {
            return ' '
        }
        
        let char_id: usize = match self.shape() {
            RailShape::Horizontal => { 0 }
            RailShape::Vertical => { 1 }
            RailShape::NorthEast => { 2 }
            RailShape::SouthEast => { 3 }
            RailShape::NorthWest => { 4 }
            RailShape::SouthWest => { 5 }
            RailShape::TNorth => { 6 }
            RailShape::TEast => { 7 }
            RailShape::TSouth => { 8 }
            RailShape::TWest => { 9 }
            RailShape::Cross => { 10 }

            _ => { unreachable!() }
        };

        match self.class() {
            RailClass::Normal => {['─', '│', '└', '┌', '┘', '┐', '┴', '├', '┬', '┤', '┼'][char_id]}
            RailClass::Highspeed => {['═', '║', '╚', '╔', '╝', '╗', '╩', '╠', '╦', '╣', '╬'][char_id]}
        }
    }
}

