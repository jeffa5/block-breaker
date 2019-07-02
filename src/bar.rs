use crate::dimensions::Dimensions;
use crate::position::Position;

/// The user controlled bar
///
/// Restricted to moving left and right. The ball bounces off this at varying angles depending on
/// the position.
pub struct Bar {
    dimensions: Dimensions,
    position: Position,
    game_dimensions: Dimensions,
}

impl Bar {
    /// Create a new bar
    pub fn new(position: Position, dimensions: Dimensions, game_dimensions: Dimensions) -> Bar {
        Bar {
            dimensions,
            position,
            game_dimensions,
        }
    }

    /// Move the bar one unit left
    pub fn move_left(&mut self) {
        if self.x() > 0 {
            self.position -= Position::new(1, 0)
        }
    }

    /// Move the bar one unit right
    pub fn move_right(&mut self) {
        if self.x() + self.width() < self.game_dimensions.width() {
            self.position += Position::new(1, 0)
        }
    }

    /// Update the game dimensions stored in the bar
    /// This clamps the bar's position if it would be outside the game on resize
    pub fn update_dimensions(&mut self, dimensions: Dimensions) {
        let mut new_position = Position::new(self.x(), dimensions.height() - 2);
        if self.x() + self.width() >= dimensions.width() {
            new_position = Position::new(dimensions.width() - self.width(), new_position.y());
        }
        self.position = new_position;

        self.game_dimensions = dimensions;
    }

    /// Get the x coordinate of the bar
    pub fn x(&self) -> u16 {
        self.position.x()
    }

    /// Get the y coordinate of the bar
    pub fn y(&self) -> u16 {
        self.position.y()
    }

    /// Get the height of the bar
    pub fn height(&self) -> u16 {
        self.dimensions.height()
    }

    /// Get the width of the bar
    pub fn width(&self) -> u16 {
        self.dimensions.width()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_position() {
        let player_bar = Bar::new(
            Position::new(0, 0),
            Dimensions::new(5, 1),
            Dimensions::new(10, 10),
        );

        assert_eq!(player_bar.position.x(), 0);
        assert_eq!(player_bar.position.y(), 8);
    }

    #[test]
    fn test_move_right_doesnt_go_over() {
        let mut player_bar = Bar::new(
            Position::new(0, 0),
            Dimensions::new(5, 1),
            Dimensions::new(10, 10),
        );

        player_bar.move_right();
        assert_eq!(player_bar.position.x(), 1);
        player_bar.move_right();
        assert_eq!(player_bar.position.x(), 2);
        player_bar.move_right();
        assert_eq!(player_bar.position.x(), 3);
        player_bar.move_right();
        assert_eq!(player_bar.position.x(), 4);
        player_bar.move_right();
        assert_eq!(player_bar.position.x(), 5);
        player_bar.move_right();
        assert_eq!(player_bar.position.x(), 5);
        player_bar.move_right();
        assert_eq!(player_bar.position.x(), 5);
    }

    #[test]
    fn test_move_left_doesnt_go_over() {
        let mut player_bar = Bar::new(
            Position::new(0, 0),
            Dimensions::new(5, 1),
            Dimensions::new(10, 10),
        );

        player_bar.move_left();
        assert_eq!(player_bar.position.x(), 0);
    }
}
