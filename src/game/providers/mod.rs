use std::rc::Rc;
use std::cell::RefCell;

use render::context::Contexts;

pub type ContextsProvider<'a> = Rc<RefCell<Contexts<'a>>>;