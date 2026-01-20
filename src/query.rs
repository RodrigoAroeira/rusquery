use std::{fmt::Display, marker::PhantomData};
pub struct Query<T> {
    inner: String,
    _type: PhantomData<T>,
}

impl<T> Query<T> {
    pub fn new(inner: String) -> Self {
        Self {
            inner,
            _type: PhantomData,
        }
    }

    pub fn get(self) -> String {
        self.inner
    }
}

impl<T> Display for Query<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
