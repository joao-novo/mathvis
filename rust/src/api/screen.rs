#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Screen {
    width: u32,
    height: u32,
}

impl Screen {
    pub fn new(width: u32, height: u32) -> Option<Self> {
        if width > 0 && height > 0 {
            return Some(Screen { width, height });
        }
        None
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
