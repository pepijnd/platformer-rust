/*
use std::path::Path;

use sdl2::surface::Surface;
use sdl2::image::LoadSurface;
use sdl2::rect::Rect;

use render::context::Contexts;
use game::Point;

use game::providers::ContextsProvider;

pub struct Sprite {
    surface: Surface<'static>,
    origin: Point,
}

trait DrawSurface {
    fn draw_surface<R1, R2>(&mut self, surface: &Surface, src: R1, dst: R2) where
        R1: Into<Option<Rect>>,
        R2: Into<Option<Rect>>;
}

impl<'a> DrawSurface for Contexts<'a> {
    fn draw_surface<R1, R2>(&mut self, surface: &Surface, src: R1, dst: R2) where
        R1: Into<Option<Rect>>,
        R2: Into<Option<Rect>> {
        let texture = self.texture_creator.create_texture_from_surface(surface).unwrap();
        self.canvas.copy(&texture, src, dst).unwrap();
    }
}


impl Sprite {
    pub fn from_file<P: AsRef<Path>>(
        path: P,
        origin: Point) -> Sprite {
        Sprite {
            surface: Surface::from_file(path).unwrap(),
            origin,
        }
    }

    pub fn draw(&self, contexts: ContextsProvider, x: i32, y: i32) {
        contexts.borrow_mut().draw_surface(&self.surface, None, Rect::new(x-self.origin.0,
                                                                          y-self.origin.1,
                                                                          self.surface.width(),
                                                                          self.surface.height()));
    }
}*/
