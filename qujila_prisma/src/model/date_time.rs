pub struct DateTimeField {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default:   Option<DateTimeValue>
}

pub struct DateTimeListField {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default: Option<  Vec<DateTimeValue>>,
}

pub struct DateTimeOptionalField {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default:   Option<DateTimeValue>,
}

pub enum DateTimeValue {
    now,
    updatedAt,
}
