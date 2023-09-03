pub struct DateTimeField {
    default: Option<DateTimeValue>
}

pub struct DateTimeListField {
    default: Option<Vec<DateTimeValue>>,
}

pub struct DateTimeOptionalField {
    default: Option<DateTimeValue>,
}

pub enum DateTimeValue {
    now,
    updatedAt,
}
