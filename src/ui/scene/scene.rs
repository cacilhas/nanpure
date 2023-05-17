use std::rc::Rc;

use super::state::State;

use chrono::Duration;

use raylib::prelude::*;

pub trait Scene {
    #[allow(unused_variables)]
    fn init(&mut self, handle: &mut RaylibHandle, thr: &RaylibThread, font: Rc<Font>);

    fn update(&mut self, dt: Duration, handle: &mut RaylibDrawHandle) -> State;
}
