use std::{cell::RefCell, rc::Rc};

use super::scene::Scene;

pub enum State {
    Keep,
    New(Rc<RefCell<dyn Scene>>),
}
