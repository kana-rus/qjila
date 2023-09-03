pub struct IntField {
    pub default: Option<IntValue>,
}

pub struct IntListField {
    pub default: Option<Vec<IntValue>>,
}

pub struct IntOptionalField {
    pub default: Option<IntValue>,
}

pub enum IntValue {
    value(i32),
    autoincrement,
}
