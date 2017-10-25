use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use time::AsMilliseconds;

use render;
use game;
use game::providers::ContextsProvider;
use game::event::EventInfoType;
use game::Point;

use std::rc::Rc;
use std::cell::RefCell;

pub struct GameVariables {
    mouse_pos: Point,
}

impl GameVariables {
    fn new() -> GameVariables {
        GameVariables {
            mouse_pos: Point(0, 0),
        }
    }

    fn set_mouse_pos(&mut self, point: Point) {
        self.mouse_pos = point;
    }

    pub fn get_mouse_pos(&self) -> game::Point {
        self.mouse_pos.clone()
    }
}

pub enum GameState {
    GameExit,
    GameRunning,
    GameStarting,
    GameStarted,
}

pub struct GameController<'a> {
    contexts: ContextsProvider<'a>,
    game_state: GameState,

    frame_counter: u64,
    frame_sub_counter: u64,
    frame_counter_timer: ::std::time::SystemTime,

    fps: f64,
    target_fps: f64,

    game_variables: Rc<RefCell<GameVariables>>,

    pub step_event: game::event::Event,
}

impl<'a> GameController<'a> {
    pub fn new(contexts: ContextsProvider<'a>, target_fps: f64) -> GameController<'a> {
        GameController {
            contexts,
            game_state: GameState::GameStarting,

            frame_counter: 0_u64,
            frame_sub_counter: 0_u64,
            frame_counter_timer: ::std::time::SystemTime::now(),

            fps: 0_f64,
            target_fps,

            game_variables: Rc::new(RefCell::new(GameVariables::new())),

            step_event: game::event::Event::new(),
        }
    }

    pub fn run(&mut self) {
        self.game_state = GameState::GameStarted;

        'gameloop: loop {
            match self.game_state {
                GameState::GameStarted |
                GameState::GameRunning => {
                    self.game_state = self.mainloop()
                }
                _ => break 'gameloop,
            };
        }
    }

    fn mainloop(&mut self) -> GameState {
        let frame_time_start = ::std::time::SystemTime::now();

        if self.frame_counter_timer.elapsed().unwrap() > ::std::time::Duration::new(1, 0) {
            let frame_target_time = self.frame_counter_timer.elapsed().unwrap();
            self.fps = self.frame_sub_counter as f64 / frame_target_time.num_milliseconds() as f64 * 1000_f64;
            self.frame_counter_timer = ::std::time::SystemTime::now();
            self.frame_sub_counter = 0;
        }

        self.contexts.borrow_mut().canvas.clear();

        let mut event_pump = self.contexts.borrow().sdl2_contexts.sdl_context.event_pump().unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } |
                Event::Quit { .. } => return GameState::GameExit,
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    ::std::thread::sleep(::std::time::Duration::from_millis(500))
                },
                Event::MouseMotion { x, y, .. } => {
                    self.game_variables.borrow_mut().set_mouse_pos(Point(x, y));
                }
                _ => {}
            }
        }

        self.begin_step();
        self.step();
        self.end_step();

        self.contexts.borrow_mut().canvas.present();

        let frame_time = frame_time_start.elapsed().unwrap();
        let frame_budget = ::std::time::Duration::from_millis((1000_f64 / self.target_fps) as u64);
        if frame_time < frame_budget {
            ::std::thread::sleep(frame_budget - frame_time);
        }

        self.frame_counter += 1;
        self.frame_sub_counter += 1;

        GameState::GameRunning
    }

    fn begin_step(&self) {

    }

    fn step(&mut self) {
        let mut event_info = Vec::new();
        event_info.push(EventInfoType::Controller(Rc::new(RefCell::new(*self))));
        event_info.push(EventInfoType::GameVariables(self.game_variables.clone()));

        self.step_event.exec(event_info);
    }

    fn end_step(&self) {

    }

    pub fn get_fps(&self) -> f64 {
        return self.fps
    }

    pub fn get_target_fps(&self) -> f64 {
        return self.target_fps
    }

    pub fn get_context_provider(&self) -> ContextsProvider<'a> {
        self.contexts.clone()
    }
}