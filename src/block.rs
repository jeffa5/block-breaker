use crate::dimensions::Dimensions;
use crate::position::Position;

pub struct Block {
    position: Position,
    dimensions: Dimensions,
    strength: u16,
}

impl Block {
    pub fn new(position: Position, dimensions: Dimensions, strength: u16) -> Block {
        Block {
            position,
            dimensions,
            strength,
        }
    }

    pub fn x(&self) -> u16 {
        self.position.x()
    }

    pub fn y(&self) -> u16 {
        self.position.y()
    }

    pub fn width(&self) -> u16 {
        self.dimensions.width()
    }

    pub fn height(&self) -> u16 {
        self.dimensions.height()
    }

    pub fn damage(&mut self, amount: u16) {
        if amount > self.strength {
            self.strength = 0
        } else {
            self.strength -= amount
        }
    }

    pub fn strength(&self) -> u16 {
        self.strength
    }
}
