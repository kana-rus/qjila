use std::marker::PhantomData;

use crate::entity::Entity;

pub struct Create<E: Entity> {
    _entity: PhantomData<fn()->E>,
}
pub struct _Create<E: Entity> {
    _entity: PhantomData<fn()->E>,
}