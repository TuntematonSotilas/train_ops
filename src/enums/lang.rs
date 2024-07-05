use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Lang {
    EN,
    FR
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}