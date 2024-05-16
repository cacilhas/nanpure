mod action;
mod gameplay;
mod helpers;
mod pause;
mod start_menu;

pub use self::action::Action;
pub use self::start_menu::StartMenu;
use std::fmt::Debug;

pub trait Scene: Debug + 'static {
    unsafe fn run_step(&mut self) -> eyre::Result<Action>;
}
