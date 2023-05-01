pub struct Limit(
    Option<usize>
);

impl Limit {
    #[inline(always)] pub(crate) fn new() -> Self {
        Self(None)
    }
    #[inline(always)] pub(crate) fn is_empty(&self) -> bool {
        self.0.is_none()
    } 
    #[inline(always)] pub(crate) fn is_set(&self) -> bool {
        self.0.is_some()
    } 
    #[inline(always)] pub(crate) fn set(&mut self, limit: usize) {
        self.0.replace(limit);
    }
}

const _: (/* Limit impls */) = {
    impl std::fmt::Display for Limit {
        #[inline(always)] fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self.0 {
                None => Ok(()),
                Some(limit) => write!(f, "LIMIT {limit}"),
            }
        }
    }
};