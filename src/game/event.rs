use game::controller::GameController;
use game::controller::GameVariables;
use game::providers::ContextsProvider;
use game::event::prelude::*;

use std::rc::Rc;
use std::cell::RefCell;

pub enum EventInfoType<'a> {
    ByEvent(Rc<RefCell<Event>>),
    Controller(Rc<RefCell<GameController<'a>>>),
    Contexts(ContextsProvider<'a>),
    GameVariables(Rc<RefCell<GameVariables>>)
}

pub struct EventInfo<'a> {
    info_types: Rc<RefCell<Vec<EventInfoType<'a>>>>,
}

impl<'a> EventInfo<'a> {
    pub fn new() -> EventInfo<'a> {
        EventInfo {
            info_types: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn add_info(&mut self, info: EventInfoType<'a>) {
        self.info_types.borrow_mut().push(info);
    }

    pub fn get_event_types(&self) -> Rc<RefCell<Vec<EventInfoType<'a>>>> {
        self.info_types.clone()
    }
}

pub struct Event {
    actions: Vec<Box<EventActionable>>
}

pub struct EventAction<T> where T: Fn(Rc<EventInfo>) {
    label: Option<String>,
    action: T
}

impl<T> EventAction<T> where T: Fn(Rc<EventInfo>) {
    pub fn new<R: Into<Option<String>>>(action: T, label: R) -> EventAction<T> {
        EventAction {
            label: label.into(),
            action,
        }
    }
}

impl<T> EventActionable for EventAction<T> where T: Fn(Rc<EventInfo>) {
    fn call(&self, info: Rc<EventInfo>) {
        (self.action)(info);
    }

    fn get_label(&self) -> String {
        self.label.clone().unwrap_or(String::new())
    }
}

impl<'a> Event {
    pub fn new() -> Event {
        Event {
            actions: Vec::new(),
        }
    }

    pub fn add_action(&mut self, action: Box<EventActionable>) {
        self.actions.push(action);
    }

    pub fn remove_action(&mut self, label: String) {
        self.actions.retain(|x| {x.get_label() != label});
    }

    pub fn exec<T: Into<Option<Vec<EventInfoType<'a>>>>>(&self, event_info: T) {
        let mut info = EventInfo::new();
        info.add_info(EventInfoType::ByEvent(Rc::new(RefCell::new(*self))));

        let event_info = event_info.into().unwrap();

        for i in event_info {
            info.add_info(i);
        }

        let info = Rc::new(info);

        for action in self.actions.iter() {
            action.call(info.clone());
        }
    }
}

mod prelude {
    use super::EventInfo;
    use std::rc::Rc;

    pub trait EventActionable {
        fn call(&self, info: Rc<EventInfo>);
        fn get_label(&self) -> String;
    }
}