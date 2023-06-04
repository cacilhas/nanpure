use std::borrow::{Borrow, BorrowMut};

use rscenes::prelude::*;

#[derive(Default)]
pub struct Resources(Option<Font>);

impl Resources {
    pub fn set(&mut self, font: Font) {
        self.0 = Some(font);
    }
}

impl Borrow<Font> for Resources {
    fn borrow(&self) -> &Font {
        match &self.0 {
            Some(font) => font,
            None => panic!("no font"),
        }
    }
}

impl BorrowMut<Font> for Resources {
    fn borrow_mut(&mut self) -> &mut Font {
        match &mut self.0 {
            Some(font) => font,
            None => panic!("no font"),
        }
    }
}
