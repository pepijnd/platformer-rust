use sdl2::ttf;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::path::Path;

use game::types::Font;
use game::controller::MainController;

pub struct FontManager<'a>
{
    font_context: &'a ttf::Sdl2TtfContext,
    fonts: HashMap<&'a str, Rc<RefCell<Font<'a, 'static>>>>,
}

impl<'a> FontManager<'a>
{
    pub fn new(font_context: &'a ttf::Sdl2TtfContext) -> FontManager<'a> {
        FontManager {
            font_context,
            fonts: HashMap::new(),
        }
    }

    pub fn load_file<P: AsRef<Path>>(&mut self, path: P, label: &'a str, size: u16)
    {
        let font = self.font_context.load_font(path, size).unwrap();
        self.fonts.insert(label, Rc::new(RefCell::new(font)));
    }

    pub fn get_font(&self, label: &'a str) -> Rc<RefCell<Font<'a, 'static>>> {
        self.fonts.get(label).unwrap().clone()
    }
}
