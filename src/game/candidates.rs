#[derive(Clone, Copy, Debug)]
pub struct Candidates(u16);

impl Candidates {
    pub fn is_set(&self, value: u8) -> bool {
        self.0 & (1 << value) != 0
    }

    pub fn set(&mut self, value: u8) {
        self.0 |= 1 << value;
    }

    pub fn clean(&mut self, value: u8) {
        self.0 &= 0b1111111110 ^ (1 << value);
    }

    pub fn only(&self) -> Option<u8> {
        let mut res: Option<u8> = None;
        for i in 1..=9u8 {
            if self.is_set(i) {
                if res.is_none() {
                    res = Some(i);
                } else {
                    return None;
                }
            }
        }
        res
    }
}

impl Default for Candidates {
    fn default() -> Self {
        Self(0b1111111110)
    }
}
