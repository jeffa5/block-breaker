type Height = u16;
type Width = u16;

#[derive(Clone)]
pub struct Dimensions {
    width: Width,
    height: Height,
}

impl Dimensions {
    pub fn new(width: Width, height: Height) -> Dimensions {
        Dimensions { width, height }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }
}
