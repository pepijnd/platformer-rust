use sdl2;

use sdl2::image::{INIT_PNG, INIT_JPG};
use sdl2::mixer::{INIT_OGG, INIT_MP3};

use game::types::Point;
use game::font::FontManager;

use std::rc::Rc;
use std::cell::RefCell;
use std::path::Path;

pub struct MainController<'a> {
    game_controller: GameController,

    font_context: sdl2::ttf::Sdl2TtfContext,
    font_manager: Option<FontManager<'a>>,

    sdl2_context: sdl2::Sdl,
    sdl2_video_subsystem: Option<sdl2::VideoSubsystem>,
    sdl2_image_context: sdl2::image::Sdl2ImageContext,
    sdl2_mixer_context: sdl2::mixer::Sdl2MixerContext,
}

pub struct GameController {
    game_variables: GameVarialbes

}

pub struct GameVarialbes {
    mouse_pos: Point,
}

impl<'a> MainController<'a> {
    pub fn new() -> MainController<'a> {
        let mut controller = MainController {
            game_controller: GameController { game_variables: GameVarialbes { mouse_pos: Point::new(0, 0) } },

            font_context: sdl2::ttf::init().unwrap(),
            font_manager: None,

            sdl2_context: sdl2::init().unwrap(),
            sdl2_video_subsystem: None,
            sdl2_image_context: sdl2::image::init(INIT_PNG | INIT_JPG).unwrap(),
            sdl2_mixer_context: sdl2::mixer::init(INIT_OGG | INIT_MP3).unwrap(),
        };
        controller.font_manager = Some(FontManager::new(&controller.font_context));
        controller.sdl2_video_subsystem = Some(controller.sdl2_context.video().unwrap());
        //controller.font_manager.load_file("assets/fonts/opensans-regular.ttf", "default", 16);

        controller
    }
}


