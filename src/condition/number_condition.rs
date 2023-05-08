use std::fmt::Display;
use super::Condition;


pub struct NumberCondition<const COLUMN: &'static str>;

pub trait Number: Display {}
impl Number for i8 {}
impl Number for i16 {}
impl Number for i32 {}
impl Number for i64 {}
impl Number for f32 {}
impl Number for f64 {}

impl<const COLUMN: &'static str> NumberCondition<COLUMN> {
    #[inline(always)] pub fn eq<N: Number>(&self, another: N) -> Condition {
        Condition(format!("{COLUMN} = {another}"))
    }
    #[inline(always)] pub fn gt<N: Number>(&self, another: N) -> Condition {
        Condition(format!("{COLUMN} > {another}"))
    }
    #[inline(always)] pub fn lt<N: Number>(&self, another: N) -> Condition {
        Condition(format!("{COLUMN} < {another}"))
    }

    #[inline(always)] pub fn ne<N: Number>(&self, another: N) -> Condition {
        Condition(format!("NOT {COLUMN} = {another}"))
    }
    #[inline(always)] pub fn ge<N: Number>(&self, another: N) -> Condition {
        Condition(format!("NOT {COLUMN} < {another}"))
    }
    #[inline(always)] pub fn le<N: Number>(&self, another: N) -> Condition {
        Condition(format!("NOT {COLUMN} > {another}"))
    }

    #[inline(always)] pub fn between<N: Number>(&self, lhs: N, rhs: N) -> Condition {
        Condition(format!("{COLUMN} BETWEEN {lhs} AND {rhs}"))
    }
    #[inline(always)] pub fn not_between<N: Number>(&self, lhs: N, rhs: N) -> Condition {
        Condition(format!("{COLUMN} NOT BETWEEN {lhs} AND {rhs}"))
    }

    pub fn is_in<'n, N: Number + 'n, L: Iterator<Item = &'n N>>(&self, list: L) -> Condition {
        Condition(format!("{COLUMN} IN ({})", {
            let mut list = list.fold(String::new(), |mut s, i| {
                s.push_str(&i.to_string());
                s.push(',');
                s
            });
            if !list.is_empty() {list.pop(/* final ',' */);}
            list
        }))
    }
    pub fn not_in<'n, N: Number + 'n, L: Iterator<Item = &'n N>>(&self, list: L) -> Condition {
        Condition(format!("{COLUMN} NOT IN ({})", {
            let mut list = list.fold(String::new(), |mut s, i| {
                s.push_str(&i.to_string());
                s.push(',');
                s
            });
            if !list.is_empty() {list.pop(/* final ',' */);}
            list
        }))
    }
}
