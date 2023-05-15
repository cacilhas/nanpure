use std::{collections::HashMap, pin::Pin};

#[derive(Default)]
pub struct Signal<'a, P> {
    index: usize,
    handles: HashMap<usize, Pin<Box<dyn Fn(&mut P) + 'a>>>,
}

impl<'a, P> Signal<'a, P> {
    pub fn register(&mut self, handle: impl Fn(&mut P) + 'a) -> Handle {
        let id = self.index;
        self.handles.insert(id, Box::pin(handle));
        self.index += 1;
        Handle(id)
    }

    pub fn unregister(&mut self, id: Handle) -> Result<(), &'static str> {
        let Handle(id) = id;
        self.handles
            .remove(&id)
            .ok_or_else(|| "the supplied id doesn’t exist")?;
        Ok(())
    }

    pub fn emit(&self, args: &mut P) {
        for handle in self.handles.values() {
            handle(args);
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Handle(usize);

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;

    #[test]
    fn it_should_register_handle() {
        let mut sig = Signal::<i32>::default();
        let id1 = sig.register(|v| println!("{v}"));
        let id2 = sig.register(|v| println!("{v}"));
        assert_eq!(sig.handles.len(), 2);
        assert_eq!(sig.index, 2);
        let Handle(num) = id1;
        assert_eq!(num, 0);
        let Handle(num) = id2;
        assert_eq!(num, 1);
    }

    #[test]
    fn it_should_unregister_handle() {
        let mut sig = Signal::<i32>::default();
        let id = sig.register(|v| println!("{v}"));
        let Handle(num) = id;
        sig.unregister(id).unwrap();
        assert!(sig.handles.is_empty());
        assert_eq!(sig.index, 1);
        assert_eq!(num, 0);
    }

    #[test]
    fn it_should_call_every_handle() {
        let mut bag1: Vec<i32> = vec![];
        let mut bag2: Vec<String> = vec![];
        {
            let mut sig = Signal::<(i32, String)>::default();
            let bag1 = Rc::new(RefCell::new(&mut bag1));
            let bag2 = Rc::new(RefCell::new(&mut bag2));
            let cloned_bag1 = Rc::clone(&bag1);
            let cloned_bag2 = Rc::clone(&bag2);
            sig.register(move |(a, _)| cloned_bag1.borrow_mut().push(*a));
            sig.register(move |(_, b)| cloned_bag2.borrow_mut().push(b.to_owned()));
            sig.emit(&mut (42, "test".to_owned()));
        }
        assert_eq!(bag1.len(), 1);
        assert_eq!(bag2.len(), 1);
        assert_eq!(bag1[0], 42);
        assert_eq!(bag2[0], "test");
    }
}
