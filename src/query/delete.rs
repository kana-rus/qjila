use std::marker::PhantomData;
use crate::{
    entity::Entity,
    condition as cond,
};


pub struct Delete<E: Entity> {
    _entity: PhantomData<fn()->E>,
    condition: cond::Condition,
    limit: cond::Limit,
    order: cond::Order,
}
pub struct _Delte<E: Entity> {
    _entity: PhantomData<fn()->E>,
    condition: cond::Condition,
    limit: cond::Limit,
}
