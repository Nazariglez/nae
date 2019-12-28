use super::{Resource, ResourceParser};
use crate::app::App;
use crate::resource_parser;
use nae_core::BaseSystem;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

/// Represent raw data
#[derive(Clone)]
pub struct Blob {
    inner: Rc<RefCell<Vec<u8>>>,
}

impl Blob {
    //https://stackoverflow.com/questions/29401626/how-do-i-return-a-reference-to-something-inside-a-refcell-without-breaking-encap
    /// Return a reference to the inner data
    pub fn data(&self) -> Ref<Vec<u8>> {
        Ref::map(self.inner.borrow(), |data| data)
    }

    /// Return a mutable referece to the inner data
    pub fn data_mut(&mut self) -> RefMut<Vec<u8>> {
        RefMut::map(self.inner.borrow_mut(), |data| data)
    }
}

impl Resource for Blob {
    type Context2d = backend::Context2d;

    fn new(file: &str) -> Self {
        Self {
            inner: Rc::new(RefCell::new(vec![])),
        }
    }

    fn parse<T>(&mut self, sys: &mut T, data: Vec<u8>) -> Result<(), String>
    where
        T: BaseSystem<Context2d = Self::Context2d>,
    {
        *self.inner.borrow_mut() = data;
        Ok(())
    }

    fn is_loaded(&self) -> bool {
        self.inner.borrow().len() != 0
    }
}

resource_parser!(Blob, backend::System);
