extern crate rand;

mod ball;
mod bar;
mod block;
mod dimensions;
mod position;
mod vector;

use ball::Ball;
use bar::Bar;
use block::Block;
use dimensions::Dimensions;
use position::Position;
use rand::Rng;
use vector::Vector;

pub struct GameState {
    ball: Ball,
    bar: Bar,
    blocks: Vec<Block>,
    dimensions: Dimensions,
    paused: bool,
}

impl GameState {
    pub fn new(width: u16, height: u16, block_density: f64) -> GameState {
        let block_dimensions = Dimensions::new(3, 1);
        let mut blocks = Vec::new();
        for x in ((width / 10)..(width * 9 / 10)).step_by(block_dimensions.width() as usize) {
            if !(x + block_dimensions.width() <= (width / 2)
                || x - block_dimensions.width() >= (width / 2))
            {
                continue;
            }
            for y in ((height / 10)..(height * 7 / 10)).step_by(block_dimensions.height() as usize)
            {
                if rand::thread_rng().gen_bool(block_density) {
                    blocks.push(Block::new(Position::new(x, y), block_dimensions.clone(), 1))
                }
            }
        }

        let game_dimensions = Dimensions::new(width, height);
        let bar_width = 10;
        GameState {
            ball: Ball::new(
                Position::new(width / 2, height / 2),
                game_dimensions.clone(),
            ),
            bar: Bar::new(
                Position::new((width / 2) - (bar_width / 2), height / 2),
                Dimensions::new(bar_width, 1),
                game_dimensions.clone(),
            ),
            blocks,
            dimensions: game_dimensions,
            paused: true,
        }
    }

    pub fn update_dimensions(&mut self, width: u16, height: u16) {
        let new_dimensions = Dimensions::new(width, height);
        self.bar.update_dimensions(new_dimensions.clone());
        self.ball.update_dimensions(new_dimensions.clone());
        self.dimensions = new_dimensions;
    }

    pub fn bar_mut(&mut self) -> &mut Bar {
        &mut self.bar
    }

    pub fn bar(&self) -> &Bar {
        &self.bar
    }

    pub fn ball_mut(&mut self) -> &mut Ball {
        &mut self.ball
    }

    pub fn ball(&self) -> &Ball {
        &self.ball
    }

    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    pub fn width(&self) -> u16 {
        self.dimensions.width()
    }

    fn collisions(&mut self) {
        // check ball with bar (simple first)
        if self.ball().y() + 1 == self.bar().y()
            && self.bar().x() <= self.ball().x()
            && self.ball().x() < self.bar().x() + self.bar().width()
        {
            // ball collides with bar
            // Instead of this simplified model the ball should bounce at angles corresponding to how far from the center of the bar it hits. The further the distance, the greater the angle
            let bar_width = self.bar().width();
            let bar_midpoint = self.bar().x() + (bar_width / 2);
            if self.ball().x() == bar_midpoint {
                self.ball_mut().bounce(Vector::new(0., -1.));
            } else if self.ball().x() > bar_midpoint {
                self.ball_mut().bounce(Vector::new(1., -3.));
            } else {
                self.ball_mut().bounce(Vector::new(-1., -3.));
            }
        }

        // check ball with top edge of window
        if self.ball().y() == 0 {
            self.ball_mut().bounce(Vector::new(0., 1.))
        }

        // check ball with right edge
        if self.ball().x() + 1 == self.dimensions.width() {
            self.ball_mut().bounce(Vector::new(-1., 0.))
        }

        // check ball with left edge
        if self.ball().x() == 0 {
            self.ball_mut().bounce(Vector::new(1., 0.))
        }

        let mut block_normal_vector = None;
        let mut block_index = None;
        // check ball with each block
        for (i, block) in self.blocks.iter().enumerate() {
            // ball collided with top edge of block
            if self.ball().y() == block.y() - 1
                && block.x() <= self.ball().x()
                && self.ball.x() < block.x() + block.width()
            {
                block_normal_vector = Some(Vector::new(0., -1.));
                block_index = Some(i);
            } else if
            // ball collided with bottom edge of block
            self.ball().y() == block.y() + block.height()
                && block.x() <= self.ball().x()
                && self.ball.x() < block.x() + block.width()
            {
                block_normal_vector = Some(Vector::new(0., 1.));
                block_index = Some(i);
            } else if
            // ball collided with right edge of block
            self.ball().x() == block.x() + block.width()
                && block.y() <= self.ball().y()
                && self.ball().y() < block.y() + block.height()
            {
                block_normal_vector = Some(Vector::new(1., 0.));
                block_index = Some(i);
            } else if
            // ball collided with left edge of block
            self.ball().x() + 1 == block.x()
                && block.y() <= self.ball().y()
                && self.ball().y() < block.y() + block.height()
            {
                block_normal_vector = Some(Vector::new(-1., 0.));
                block_index = Some(i);
            }

            if self.ball().x() >= block.x()
                && self.ball().x() < block.x() + block.width()
                && self.ball().y() >= block.y()
                && self.ball().y() < block.y() + block.height()
            {
                // ball collides with this block
                block_normal_vector = Some(Vector::new(1., 1.));
                block_index = Some(i);
            }
        }
        if let Some(normal) = block_normal_vector {
            self.ball_mut().bounce(normal)
        };
        if let Some(i) = block_index {
            let power = self.ball().power();
            self.blocks[i].damage(power);
            if self.blocks[i].strength() == 0 {
                self.blocks.remove(i);
            }
        }
    }

    pub fn tick(&mut self) {
        if !self.paused && !self.game_over() {
            self.ball_mut().tick();
            self.collisions();
        }
    }

    pub fn game_over(&self) -> bool {
        self.ball().y() >= self.dimensions.height() - 1
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn pause(&mut self) {
        self.paused = true
    }

    pub fn unpause(&mut self) {
        self.paused = false
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused
    }
}
