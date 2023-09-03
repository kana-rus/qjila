pub struct IntField {
    default: Option<i32>,
}

pub struct IntListField {
    default: Option<Vec<i32>>,
}

pub struct IntOptionalField {
    default: Option<i32>,
}

pub enum IntValue {
    value(i32),
    autoincrement,
}
