pub struct EnumField {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default:   Option<String>,
}

pub struct EnumListField {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default:   Option<Vec<String>>,
}

pub struct EnumOptionalField {
    pub map:       Option<String>,
    pub id:        Option<()>,
    pub unique:    Option<()>,
    pub default:   Option<String>,
}
