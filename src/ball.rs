use crate::dimensions::Dimensions;
use crate::position::Position;
use crate::vector::Vector;

pub struct Ball {
    position: Position,
    velocity: Vector,
    power: u16,
    game_dimensions: Dimensions,
}

impl Ball {
    pub fn new(pos: Position, game_dimensions: Dimensions) -> Ball {
        Ball {
            position: pos,
            velocity: Vector::new(0., 1.),
            power: 1,
            game_dimensions,
        }
    }

    pub fn x(&self) -> u16 {
        self.position.x()
    }

    pub fn y(&self) -> u16 {
        self.position.y()
    }

    pub fn power(&self) -> u16 {
        self.power
    }

    pub fn update_dimensions(&mut self, dimensions: Dimensions) {
        self.game_dimensions = dimensions;
        if self.x() >= self.game_dimensions.width() {
            self.position = Position::new(self.game_dimensions.width() - 1, self.y())
        }
        if self.y() >= self.game_dimensions.height() {
            self.position = Position::new(self.x(), self.game_dimensions.height() - 1)
        }
    }

    pub fn tick(&mut self) {
        let mut new_position = &self.position + &self.velocity;

        if new_position.x() >= self.game_dimensions.width() {
            new_position = Position::new(self.game_dimensions.width() - 1, new_position.y())
        }
        if new_position.y() >= self.game_dimensions.height() {
            new_position = Position::new(new_position.x(), self.game_dimensions.height())
        }
        if new_position.y() >= self.game_dimensions.height() - 1 {
            self.velocity = Vector::new(0., 0.)
        }
        self.position = new_position
    }

    pub fn bounce(&mut self, normal: Vector) {
        self.velocity -= 2. * self.velocity.dot(&normal) * normal;
        self.velocity.normalise()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounce_works() {
        let mut ball = Ball::new(Position::new(0, 0), Dimensions::new(100, 100));

        // ball is initially travelling directly down
        // bouncing in (0, -1) should make it's velocity (0, -1)
        ball.bounce(Vector::new(0., -1.));
        assert_eq!(ball.velocity, Vector::new(0., -1.));
    }
}
