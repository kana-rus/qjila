pub struct IntField {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default: Option<IntValue>,
}

pub struct IntListField {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default: Option<Vec<IntValue>>,
}

pub struct IntOptionalField {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default: Option<IntValue>,
}

pub enum IntValue {
    value(i32),
    autoincrement,
}
