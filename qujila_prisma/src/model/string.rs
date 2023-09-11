pub struct StringField {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default:   Option<StringValue>,
}

pub struct StringListField {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default:   Option<Vec<StringValue>>,
}

pub struct StringOptionalField {
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
