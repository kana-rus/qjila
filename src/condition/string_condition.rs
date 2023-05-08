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
    #[inline(always)] pub fn eq<S: Str>(&self, another: S) -> Condition {
        Condition(format!("{COLUMN} = '{}'", another.as_str()))
    }
    #[inline(always)] pub fn like<S: Str>(&self, another: S) -> Condition {
        Condition(format!("{COLUMN} LIKE '{}'", another.as_str()))
    }

    #[inline(always)] pub fn ne<S: Str>(&self, another: S) -> Condition {
        Condition(format!("NOT {COLUMN} = '{}'", another.as_str()))
    }
    #[inline(always)] pub fn unlike<S: Str>(&self, another: S) -> Condition {
        Condition(format!("NOT {COLUMN} LIKE '{}'", another.as_str()))
    }

    pub fn is_in<'s, S: Str + 's, L: Iterator<Item = &'s S>>(&self, list: L) -> Condition {
        Condition(format!("{COLUMN} IN ({})", {
            let mut list = list.fold(String::new(), |mut l, s| {
                l.push('\'');
                l.push_str(s.as_str());
                l.push('\'');
                l.push(',');
                l
            });
            if !list.is_empty() {list.pop(/* final ',' */);}
            list
        }))
    }
    pub fn not_in<'s, S: Str + 's, L: Iterator<Item = &'s S>>(&self, list: L) -> Condition {
        Condition(format!("{COLUMN} NOT IN ({})", {
            let mut list = list.fold(String::new(), |mut l, s| {
                l.push('\'');
                l.push_str(s.as_str());
                l.push('\'');
                l.push(',');
                l
            });
            if !list.is_empty() {list.pop(/* final ',' */);}
            list
        }))
    }
}
