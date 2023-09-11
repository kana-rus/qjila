pub struct DecimalField {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default:   Option<i128>,
}

pub struct DecimalListField {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default:   Option<Vec<i128>>,
}

pub struct DecimalOptionalField {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default:   Option<i128>,
}
