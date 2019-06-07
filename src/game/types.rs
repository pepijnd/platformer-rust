use sdl2::ttf;

pub type FontContext = ttf::Sdl2TtfContext;
pub type Font<'a, 'b> = ttf::Font<'a, 'b>;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y:i32) -> Point {
        Point {
            x,
            y,
        }
    }

    pub fn get(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}