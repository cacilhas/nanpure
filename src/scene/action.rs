use super::Scene;

#[derive(Debug)]
pub enum Action {
    Keep,
    Pop(usize),
    Push(Box<dyn Scene>),
}
