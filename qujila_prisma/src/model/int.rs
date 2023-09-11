pub struct IntAttributes {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default: Option<IntValue>,
}

pub struct IntListAttributes {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default: Option<Vec<IntValue>>,
}

pub struct IntOptionalAttributes {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default: Option<IntValue>,
}

pub enum IntValue {
    value(i32),
    autoincrement,
}
