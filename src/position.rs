use crate::vector::Vector;
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// A generic position
pub struct Position {
    x: f64,
    y: f64,
}

impl Position {
    /// Create a new position
    pub fn new(x: u16, y: u16) -> Position {
        Position {
            x: f64::from(x),
            y: f64::from(y),
        }
    }

    /// Get the x coordinate
    pub fn x(&self) -> u16 {
        self.x as u16
    }

    /// Get the y coordinate
    pub fn y(&self) -> u16 {
        self.y as u16
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a, 'b> Add<&'b Position> for &'a Position {
    type Output = Position;

    fn add(self, other: &'b Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a, 'b> Add<&'b Vector> for &'a Position {
    type Output = Position;

    fn add(self, other: &'b Vector) -> Position {
        Position {
            x: self.x + other.x(),
            y: self.y + other.y(),
        }
    }
}

impl AddAssign<Position> for Position {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a> AddAssign<&'a Position> for Position {
    fn add_assign(&mut self, other: &Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
