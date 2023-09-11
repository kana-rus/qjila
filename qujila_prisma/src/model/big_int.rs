pub struct BigIntAttributes {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default:   Option<BigIntValue>,
}

pub struct BigIntListAttributes {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default:   Option<Vec<BigIntValue>>,
}

pub struct BigIntOptionalAttributes {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default:   Option<BigIntValue>,
}

pub enum BigIntValue {
    value(i64),
    autoincrement,
}
