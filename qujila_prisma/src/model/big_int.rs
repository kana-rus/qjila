pub struct BigIntField {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default:   Option<BigIntValue>,
}

pub struct BigIntListField {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default:   Option<Vec<BigIntValue>>,
}

pub struct BigIntOptionalField {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default:   Option<BigIntValue>,
}

pub enum BigIntValue {
    value(i64),
    autoincrement,
}
