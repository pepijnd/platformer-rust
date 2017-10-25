use sdl2;
use sdl2::image::{INIT_PNG, INIT_JPG};
use sdl2::mixer::{INIT_OGG, INIT_MP3};

use game::font::FontManager;

pub struct Settings {
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
}

pub struct Sdl2Contexts<'a> {
    pub sdl_context: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub font_manager: FontManager<'a>,
    pub image_context: sdl2::image::Sdl2ImageContext,
    pub mixer_context: sdl2::mixer::Sdl2MixerContext,
}

pub struct Contexts<'a> {

    pub canvas: sdl2::render::WindowCanvas,
    pub texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    pub sdl2_contexts: Sdl2Contexts<'a>,
}

impl<'a> Sdl2Contexts<'a> {
    pub fn create_contexts(self, settings: Settings) -> Contexts<'a> {
        let window = self.video_subsystem
                         .window(&settings.window_title, settings.window_width, settings.window_height)
                         .position_centered()
                         .build().unwrap();
        let canvas = window.into_canvas().software().build().unwrap();
        let texture_creator = canvas.texture_creator();

        Contexts {
            canvas,
            texture_creator,
            sdl2_contexts: self,
        }
    }
}

trait CreateFontManager<'a> {
    fn font_manager(self) -> FontManager<'a>;
}

impl<'a> CreateFontManager<'a> for sdl2::ttf::Sdl2TtfContext {
    fn font_manager(self) -> FontManager<'a> {
        FontManager::new(self)
    }
}

pub fn init_sdl2<'a>() -> Sdl2Contexts<'a> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let font_context = sdl2::ttf::init().unwrap();
    let font_manager = font_context.font_manager();
    let image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();
    let mixer_context = sdl2::mixer::init(INIT_OGG | INIT_MP3).unwrap();

    Sdl2Contexts {
        sdl_context,
        video_subsystem,
        font_manager,
        image_context,
        mixer_context,
    }
}