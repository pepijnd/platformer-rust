use sprites::Sprite;
use game::Point;
use game::providers::ContextsProvider;
use std::path::Path;

pub struct SimpleObject {
    xpos: f64,
    ypos: f64,
    sprite: Sprite,
}

impl SimpleObject {
    pub fn new<P: AsRef<Path>>(contexts: ContextsProvider,
                                   path: P) -> SimpleObject {
        let sprite = Sprite::from_file(path, Point(232, 212));
        SimpleObject {
            xpos: 0.0,
            ypos: 0.0,
            sprite,
        }
    }

    pub fn set_pos(&mut self, xpos: f64, ypos: f64) {
        self.xpos = xpos;
        self.ypos = ypos;
    }

    pub fn draw_sprite(&self, contexts: ContextsProvider) {
        self.sprite.draw(contexts, self.xpos as i32, self.ypos as i32)
    }
}
