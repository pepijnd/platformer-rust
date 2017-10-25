#![feature(std_misd)]
#![feature(refcell_replace_swap)]

extern crate sdl2;

mod render;
mod game;
mod time;
mod sprites;

use render::context;
use game::controller::GameController;
use game::object::SimpleObject;
use game::event::EventInfoType;
use game::providers::ContextsProvider;

use std::cell::RefCell;
use std::rc::Rc;
use std::io::Write;
use std::io::stdout;

fn main() {
    let sdl2 = context::init_sdl2();
    let contexts = Rc::new(RefCell::new(sdl2.create_contexts(context::Settings {
        window_title: String::from("Platformer"),
        window_width: 800,
        window_height: 600,
    })));


    let obj = Rc::new(RefCell::new(SimpleObject::new(contexts.clone(),
                                       ::std::path::Path::new("assets/sprites/kirby.png"))));

    let mut game_controller = GameController::new(contexts.clone(), 50_f64);

    println!();

    let test = game_controller.get_context_provider();
    let val = test.borrow_mut();
    val.sdl2_contexts.font_manager.load_file("assests/fonts/OpenSans-Regular.ttf", "opensans", 16);

    game_controller.step_event.add_action(Box::new(game::event::EventAction::new( move |event_info| {
        let info_types = event_info.get_event_types();
        for info in info_types.borrow().iter() {
            match *info {
                EventInfoType::Controller(ref controller) => {
                    let fps = controller.borrow().get_fps();
                    let font = contexts.borrow().sdl2_contexts.font_manager.get_font("opensans");
                }
                EventInfoType::GameVariables(ref variables) => {
                    let mouse_pos = variables.borrow().get_mouse_pos();
                    obj.borrow_mut().set_pos(mouse_pos.0 as f64, mouse_pos.1 as f64);
                }
                _ => {}
            }
        }

        obj.borrow().draw_sprite(contexts.clone());
    }, String::from("obj_draw"))));

    game_controller.run();
}
