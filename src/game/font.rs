use sdl2::ttf;

use game::providers::ContextsProvider;
use render::context::Sdl2Contexts;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::path::Path;

pub type Font<'a> = ttf::Font<'a, 'static>;

pub struct FontManager<'a> {
    pub font_context: ttf::Sdl2TtfContext,
    fonts: HashMap<&'a str, Rc<RefCell<Font<'a>>>>,
}

impl<'a> FontManager<'a> {
    pub fn new(font_context: ttf::Sdl2TtfContext) -> FontManager<'a> {
        FontManager {
            font_context,
            fonts: HashMap::new(),
        }
    }

    pub fn load_file<P: AsRef<Path>>(&'a mut self, path: P, label: &'static str, size: u16) {
        let font = self.font_context.load_font(path, size).unwrap();
        self.fonts.insert(label, Rc::new(RefCell::new(font)));
    }

    pub fn get_font(&self, label: &'a str) -> Rc<RefCell<Font<'a>>> {
        self.fonts.get(label).unwrap().clone()
    }
}