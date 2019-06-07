#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate sdl2;

mod game;
mod time;
mod sprites;

use game::controller;

use std::cell::RefCell;
use std::rc::Rc;
use std::io::Write;
use std::io::stdout;

fn main() {
    /*
    let window = SDL2_VIDEO_SUBSYSTEM
        .window("test", 800, 600)
        .position_centered()
        .build().unwrap();
    let canvas = window.into_canvas().software().build().unwrap();
    let texture_creator = canvas.texture_creator();*/

    let mut main_controller = game::controller::MainController::new();
}
