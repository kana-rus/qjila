pub struct Limit(
    Option<usize>
);

impl Limit {
    #[inline] pub(crate) fn new() -> Self {
        Self(None)
    }
    #[inline] pub(crate) fn set(&mut self, limit: usize) {
        self.0.replace(limit);
    }
}

const _: (/* Limit impls */) = {
    impl std::fmt::Display for Limit {
        #[inline] fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self.0 {
                None => Ok(()),
                Some(limit) => write!(f, "LIMIT {limit}"),
            }
        }
    }
};