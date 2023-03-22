mod order; pub(crate) use order::Order;
mod limit; pub(crate) use limit::Limit;
mod number_condition; pub(crate) use number_condition::NumberCondition;
mod string_condition; pub(crate) use string_condition::StringCondition;


pub struct Condition(
    String
);

impl Condition {
    #[inline] pub fn AND(self, another: Self) -> Condition {
        Condition(format!(
            "({} AND {})", self.0, another.0
        ))
    }
    #[inline] pub fn OR(self, another: Self) -> Condition {
        Condition(format!(
            "({} OR {})", self.0, another.0
        ))
    }
    #[inline] pub fn new() -> Self {
        Self(String::new())
    }
}

const _: (/* Ccndition impls */) = {
    impl std::ops::BitOr for Condition {
        type Output = Condition;
        #[inline] fn bitor(self, rhs: Self) -> Self::Output {
            self.OR(rhs)
        }
    }
    impl std::ops::BitAnd for Condition {
        type Output = Condition;
        #[inline] fn bitand(self, rhs: Self) -> Self::Output {
            self.AND(rhs)
        }
    }

    impl<const N: usize> Into<Condition> for [Condition; N] {
        #[inline] fn into(self) -> Condition {
            self.into_iter().fold(Condition::new(), |it, next| it.AND(next))
        }
    }
    
    impl std::fmt::Display for Condition {
        #[inline] fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.0.is_empty() {Ok(())} else {write!(f, "WHERE {}", self.0)}
        }
    }
};
