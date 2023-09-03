pub struct DateTimeField {
    pub default: Option<DateTimeValue>
}

pub struct DateTimeListField {
    pub default: Option<Vec<DateTimeValue>>,
}

pub struct DateTimeOptionalField {
    pub default: Option<DateTimeValue>,
}

pub enum DateTimeValue {
    now,
    updatedAt,
}
