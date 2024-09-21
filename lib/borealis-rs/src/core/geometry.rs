#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

#[derive(Debug)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

#[derive(Debug)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

impl Rect {
    pub fn new(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }

    pub fn min_x(&self) -> f32 {
        self.origin.x
    }

    pub fn min_y(&self) -> f32 {
        self.origin.y
    }

    pub fn mid_x(&self) -> f32 {
        self.origin.x + self.width() / 2.0
    }

    pub fn mid_y(&self) -> f32 {
        self.origin.y + self.height() / 2.0
    }

    pub fn max_x(&self) -> f32 {
        self.origin.x + self.width()
    }

    pub fn max_y(&self) -> f32 {
        self.origin.y + self.height()
    }

    pub fn width(&self) -> f32 {
        self.size.width
    }

    pub fn height(&self) -> f32 {
        self.size.height
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self::new(Point::default(), Size::default())
    }
}
