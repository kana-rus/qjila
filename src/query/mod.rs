mod query_count; pub use query_count::{count};
mod query_exists; pub use query_exists::{exists};
mod query_is_single; pub use query_is_single::{is_single};

mod query_first; pub use query_first::{First};
mod query_all; pub use query_all::{All};
mod query_single; pub use query_single::{Single};
mod query_search; pub use query_search::{Search};

mod query_create; pub use query_create::{create, Create};
mod query_delete; pub use query_delete::{delete, Delete};
mod query_update; pub use query_update::{update, Update};


const Q_SIZE: usize = 16;
pub(crate) struct Q<T> {
    len: usize,
    elements: [Option<T>; Q_SIZE]
}
impl<T> Q<T> {
    #[inline] fn new() -> Self {
        Self { len: 0, elements: [
            None, None, None, None,
            None, None, None, None,
            None, None, None, None,
            None, None, None, None,
        ] }
    }
    #[inline] fn is_empty(&self) -> bool {
        self.len == 0
    }
    /// push an element without checking current length
    #[inline] unsafe fn push(&mut self, element: T) {
        self.elements[self.len].replace(element);
        self.len += 1;
    }
}
const _: () = {
    impl<'i, T> Iterator for &'i Q<T> {
        type Item = &'i T;
        fn next(&mut self) -> Option<Self::Item> {
            (self.elements[0..self.len]).iter()
                .map(|e| unsafe { e.as_ref().unwrap_unchecked() })
                .next()
        }
    }
};

impl Q<&'static str> {
    fn join(&self, separater: &'static str) -> String {
        match self.len {
            0 => String::new(),
            _ => {
                let mut joined = String::with_capacity(3 * self.len + 1);
                joined.push_str(unsafe { self.elements[0].unwrap_unchecked() });
                for e in &self.elements[1..self.len] {
                    joined.push_str(separater);
                    joined.push_str(unsafe { e.as_ref().unwrap_unchecked() });
                }
                joined
            }
        }
    }
} impl Q<String> {
    fn join(&self, separater: &'static str) -> String {
        match self.len {
            0 => String::new(),
            _ => {
                let mut joined = String::with_capacity(3 * self.len + 1);
                joined.push_str(unsafe { (&self.elements[0]).as_ref().unwrap_unchecked() });
                for e in &self.elements[1..self.len] {
                    joined.push_str(separater);
                    joined.push_str(unsafe { e.as_ref().unwrap_unchecked() });
                }
                joined
            }
        }
    }
}
