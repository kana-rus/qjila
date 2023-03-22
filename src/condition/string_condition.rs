use super::Condition;


pub struct StringCondition<const COLUMN: &'static str>;

pub trait Str {
    fn as_str(&self) -> &str;
    fn to_string(self) -> String;
}
impl Str for String {
    fn as_str(&self) -> &str {&self}
    fn to_string(self) -> String {self}
}
impl Str for &str {
    fn as_str(&self) -> &str {self}
    fn to_string(self) -> String {self.to_owned()}
}

impl<const COLUMN: &'static str> StringCondition<COLUMN> {
    pub fn eq<S: Str>(&self, another: S) -> Condition {
        Condition(format!("{COLUMN} = '{}'", another.as_str()))
    }
    pub fn like<S: Str>(&self, another: S) -> Condition {
        Condition(format!("{COLUMN} LIKE '{}'", another.as_str()))
    }

    pub fn ne<S: Str>(&self, another: S) -> Condition {
        Condition(format!("NOT {COLUMN} = '{}'", another.as_str()))
    }
    pub fn unlike<S: Str>(&self, another: S) -> Condition {
        Condition(format!("NOT {COLUMN} LIKE '{}'", another.as_str()))
    }
}
