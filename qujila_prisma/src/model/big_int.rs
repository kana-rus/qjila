pub struct BigIntField {
    default: Option<i64>,
}

pub struct BigIntListField {
    default: Option<Vec<i64>>,
}

pub struct BigIntOptionalField {
    default: Option<i64>,
}

pub enum BigIntValue {
    value(i64),
    autoincrement,
}
