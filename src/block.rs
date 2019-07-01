use crate::dimensions::Dimensions;
use crate::position::Position;

/// A single block in the game
///
/// Each block has a position, size and strength
pub struct Block {
    position: Position,
    dimensions: Dimensions,
    strength: u16,
}

impl Block {
    /// Create a new block
    pub fn new(position: Position, dimensions: Dimensions, strength: u16) -> Block {
        Block {
            position,
            dimensions,
            strength,
        }
    }

    /// Get the x coordinate of the block
    pub fn x(&self) -> u16 {
        self.position.x()
    }

    /// Get the y coordinate of the block
    pub fn y(&self) -> u16 {
        self.position.y()
    }

    /// Get the width of the block
    pub fn width(&self) -> u16 {
        self.dimensions.width()
    }

    /// Get the height of the block
    pub fn height(&self) -> u16 {
        self.dimensions.height()
    }

    /// Damage the block, from a contact with the ball
    pub fn damage(&mut self, amount: u16) {
        if amount > self.strength {
            self.strength = 0
        } else {
            self.strength -= amount
        }
    }

    /// Get the strength of the block
    pub fn strength(&self) -> u16 {
        self.strength
    }
}
