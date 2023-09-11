pub struct DateTimeAttributes {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default:   Option<DateTimeValue>
}

pub struct DateTimeListAttributes {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default: Option<  Vec<DateTimeValue>>,
}

pub struct DateTimeOptionalAttributes {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default:   Option<DateTimeValue>,
}

pub enum DateTimeValue {
    now,
    updatedAt,
}
