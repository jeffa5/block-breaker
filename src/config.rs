/// A struct for holding the config
pub struct Config {
    /// The density to generate blocks at
    pub block_density: f64,
    /// The strength of each block
    pub block_strength: u16,
    /// The width of the bar
    pub bar_width: u16,
    /// The strength of the ball
    pub ball_power: u16,
}

impl Config {
    /// Create a new config
    pub fn new(block_density: f64, block_strength: u16, bar_width: u16, ball_power: u16) -> Self {
        Self {
            block_density,
            block_strength,
            bar_width,
            ball_power,
        }
    }
}
