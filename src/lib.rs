//! # Block Breaker
//!
//! A library for the block breaker game.

extern crate rand;

mod ball;
mod bar;
mod block;
mod config;
mod dimensions;
mod gamestate;
mod position;
mod vector;

pub use ball::Ball;
pub use bar::Bar;
pub use block::Block;
pub use config::Config;
pub use dimensions::Dimensions;
pub use gamestate::GameState;
pub use position::Position;
pub use vector::Vector;
