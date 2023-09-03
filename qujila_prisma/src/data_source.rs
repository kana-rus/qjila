
pub struct DataSource {
    pub name:     String,
    pub provider: Provider,
    pub url:      String,
}

pub enum Provider {
    postgresql,
    mysql,
    sqlite,
}
