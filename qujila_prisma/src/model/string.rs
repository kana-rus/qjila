pub struct StringAttributes {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default:   Option<StringValue>,
}

pub struct StringListAttributes {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default:   Option<Vec<StringValue>>,
}

pub struct StringOptionalAttributes {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default:   Option<StringValue>,
}

pub enum StringValue {
    value(String),
    cuid,
    uuid,
}
