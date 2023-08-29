#![allow(non_camel_case_types)]

#![allow(incomplete_features)]
#![feature(adt_const_params)]

use std::marker::{PhantomData};


pub mod c {
    pub struct String  <const C: super::Constraints<{super::DBType::String}>   = {super::c()}>;
    pub struct usize   <const C: super::Constraints<{super::DBType::usize}>    = {super::c()}>;
    pub struct DateTime<const C: super::Constraints<{super::DBType::DateTime}> = {super::c()}>;
}

#[derive(::std::marker::ConstParamTy, PartialEq, Eq)]
pub enum DBType { String, usize, DateTime }

#[derive(::std::marker::ConstParamTy, PartialEq, Eq)]
pub struct Constraints<const T: DBType>;
impl Constraints<{DBType::usize}> {
    const fn increment(self) -> Self {self}
}

pub const fn c<const T: DBType>() -> Constraints<T> {Constraints}




#[cfg(test)] fn __main() {
    struct User {
        id: c::usize::<{c().increment()}>,
    }
}
