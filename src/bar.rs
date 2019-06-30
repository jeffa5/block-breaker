use crate::dimensions::Dimensions;
use crate::position::Position;

pub struct Bar {
    dimensions: Dimensions,
    position: Position,
    game_dimensions: Dimensions,
}

impl Bar {
    pub fn new(position: Position, dimensions: Dimensions, game_dimensions: Dimensions) -> Bar {
        Bar {
            dimensions,
            position,
            game_dimensions,
        }
    }

    pub fn move_left(&mut self) {
        if self.x() > 0 {
            self.position -= Position::new(1, 0)
        }
    }

    pub fn move_right(&mut self) {
        if self.x() + self.width() + 1 < self.game_dimensions.width() {
            self.position += Position::new(1, 0)
        }
    }

    pub fn update_dimensions(&mut self, dimensions: Dimensions) {
        let mut new_position = Position::new(self.x(), dimensions.height() - 2);
        if self.x() + self.width() + 1 >= dimensions.width() {
            new_position += Position::new(dimensions.width() - self.width() - 1, 0)
        }
        self.position = new_position;

        self.game_dimensions = dimensions;
    }

    pub fn x(&self) -> u16 {
        self.position.x()
    }

    pub fn y(&self) -> u16 {
        self.position.y()
    }

    pub fn height(&self) -> u16 {
        self.dimensions.height()
    }

    pub fn width(&self) -> u16 {
        self.dimensions.width()
    }
}
