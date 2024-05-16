#[derive(Clone, Copy, Debug)]
pub struct Candidates(u16);

impl Candidates {
    pub fn is_set(&self, value: u8) -> bool {
        self.0 & (1 << value) != 0
    }

    pub fn get_single_value(&self) -> Option<u8> {
        for value in 1_u8..=9 {
            if self.0 == 1 << value {
                return Some(value);
            }
        }
        None
    }

    pub fn set(&mut self, value: u8) {
        self.0 |= 1 << value;
    }

    pub fn clean(&mut self, value: u8) {
        self.0 &= 0b1111111110 ^ (1 << value);
    }
}

impl Default for Candidates {
    fn default() -> Self {
        Self(0b1111111110)
    }
}
