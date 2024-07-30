use std::fmt;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Lang {
    #[default]
    EN,
    ES,
    FR,
    DE
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}