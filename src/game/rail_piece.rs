use crate::game::direction::Direction;
use crate::game::rail_type::{RailClass, RailShape, RailType};

#[derive(Copy, Clone, PartialEq)]
pub struct RailPiece {
    rail_type: RailType,
}

impl RailPiece {
    pub fn new(shape: RailShape, class: RailClass) -> Self {
        Self { rail_type: RailType::new(shape, class) }
    }
    
    pub fn can_connect(&self, direction: Direction) -> bool {
        self.rail_type.connections()[usize::from(direction)]
    }
    
    pub fn can_connect_with_piece(&self, other: &RailPiece, direction: Direction) -> bool {
        self.can_connect(direction) && other.can_connect(direction.opposite())
    }
}