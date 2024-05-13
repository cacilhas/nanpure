mod start_menu;

pub use self::start_menu::StartMenu;
use std::fmt::Debug;

pub trait Scene: Debug + 'static {
    unsafe fn run_step(&mut self) -> Action;
}

#[derive(Debug)]
pub enum Action {
    Keep,
    Pop(usize),
    Push(Box<dyn Scene>),
}
