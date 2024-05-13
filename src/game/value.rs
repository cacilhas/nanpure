#[derive(Clone, Copy, Debug, Default)]
pub struct Value(Option<u8>);

impl Value {
    pub fn new(value: u8) -> Self {
        Self(Some(value))
    }

    pub fn clean(&mut self) {
        self.0 = None;
    }

    pub fn insert(&mut self, value: u8) {
        let _ = self.0.insert(value);
    }

    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }
}

impl From<Value> for Option<u8> {
    fn from(value: Value) -> Self {
        value.0
    }
}

impl From<Value> for u8 {
    fn from(value: Value) -> Self {
        value.0.unwrap()
    }
}
