use std::marker::PhantomData;
use crate::{
    entity::Entity,
    condition as cond,
};


pub struct Update<E: Entity> {
    _entity: PhantomData<fn()->E>,
    condition: cond::Condition,
    limit: cond::Limit,
    order: cond::Order,
}
pub struct _Update<E: Entity> {
    _entity: PhantomData<fn()->E>,
    condition: cond::Condition,
    limit: cond::Limit,
}
