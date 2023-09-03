pub struct BigIntField {
    pub default: Option<BigIntValue>,
}

pub struct BigIntListField {
    pub default: Option<Vec<BigIntValue>>,
}

pub struct BigIntOptionalField {
    pub default: Option<BigIntValue>,
}

pub enum BigIntValue {
    value(i64),
    autoincrement,
}
