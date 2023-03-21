use std::marker::PhantomData;

use crate::{entity::Entity, condition::Condition};


pub struct Count<E: Entity>{
    condition: Condition,
    _entity: PhantomData<fn()->E>,
}

pub struct First<E: Entity> {
    _entity: PhantomData<fn()->E>,
}
