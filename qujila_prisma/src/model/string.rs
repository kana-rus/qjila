pub struct StringField {
    pub default: Option<StringValue>,
}

pub struct StringListField {
    pub default: Option<Vec<StringValue>>,
}

pub struct StringOptionalField {
    pub default: Option<StringValue>,
}

pub enum StringValue {
    value(String),
    cuid,
    uuid,
}
