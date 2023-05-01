pub struct Order(
    String
);

impl Order {
    #[inline(always)] pub(crate) fn new() -> Self {
        Self(String::new())
    }
    #[inline(always)] pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    #[inline(always)] pub(crate) fn is_set(&self) -> bool {
        !self.0.is_empty()
    }
    #[inline(always)] pub(crate) fn ASC(&mut self, by: &'static str) {
        self.0 += " ORDER BY ";
        self.0 += by;
    }
    #[inline(always)] pub(crate) fn DESC(&mut self, by: &'static str) {
        self.0 += " ORDER BY ";
        self.0 += by;
        self.0 += " DESC";
    }
}

const _: (/* Order impls */) = {
    impl std::fmt::Display for Order {
        #[inline(always)] fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
};
