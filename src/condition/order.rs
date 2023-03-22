pub struct Order(
    String
);

impl Order {
    #[inline] pub(crate) fn new() -> Self {
        Self(String::new())
    }
    #[inline] pub(crate) fn ASC(&mut self, by: &'static str) {
        self.0 += " ORDER BY ";
        self.0 += by;
    }
    #[inline] pub(crate) fn DESC(&mut self, by: &'static str) {
        self.0 += " ORDER BY ";
        self.0 += by;
        self.0 += " DESC";
    }
}

const _: (/* Order impls */) = {
    impl std::fmt::Display for Order {
        #[inline] fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
};
