use crate::position::Position;
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

/// A 2D vector
#[derive(Debug, PartialEq)]
pub struct Vector {
    x: f64,
    y: f64,
}

impl Vector {
    /// Create a new vector
    pub fn new(x: f64, y: f64) -> Vector {
        let mut v = Vector { x, y };
        v.normalise();
        v
    }

    /// Get the x value
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Get the y value
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Perform the inner product with another vector
    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Negate the vector, negating each of the components
    pub fn negate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
    }

    /// Get the length (magnitude) of the vector
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    /// Normalise the vector
    pub fn normalise(&mut self) {
        let magnitude = self.magnitude();
        self.x /= magnitude;
        self.y /= magnitude;
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a, 'b> Add<&'b Vector> for &'a Vector {
    type Output = Vector;

    fn add(self, other: &'b Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a, 'b> Add<&'b Position> for &'a Vector {
    type Output = Position;

    fn add(self, other: &'b Position) -> Position {
        Position::new(
            (self.x + f64::from(other.x())) as u16,
            (self.y + f64::from(other.y())) as u16,
        )
    }
}

impl AddAssign<Vector> for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a> AddAssign<&'a Vector> for Vector {
    fn add_assign(&mut self, other: &Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, scalar: f64) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, vector: Vector) -> Vector {
        Vector {
            x: self * vector.x,
            y: self * vector.y,
        }
    }
}
