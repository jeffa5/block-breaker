type Height = u16;
type Width = u16;

/// Generic dimensions
#[derive(Clone, Debug)]
pub struct Dimensions {
    width: Width,
    height: Height,
}

impl Dimensions {
    /// Create a new set of dimensions
    pub fn new(width: Width, height: Height) -> Dimensions {
        Dimensions { width, height }
    }

    /// Get the width
    pub fn width(&self) -> Width {
        self.width
    }

    /// Get the height
    pub fn height(&self) -> Height {
        self.height
    }
}
