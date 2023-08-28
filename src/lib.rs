#![allow(non_camel_case_types)]
use std::marker::PhantomData;


pub trait DBValue {}

pub mod c {
    pub struct String; impl super::DBValue for String {}
    pub struct usize; impl super::DBValue for usize {}
    pub struct DateTime; impl super::DBValue for DateTime {}
}

pub struct Constraints<V: DBValue>(PhantomData<fn()->V>);

pub fn c<V: DBValue>() -> Constraints<V> {Constraints(PhantomData)}
