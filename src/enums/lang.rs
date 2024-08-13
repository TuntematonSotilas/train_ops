#[derive(Clone, PartialEq, Default)]
pub enum Lang {
    #[default]
    EN,
    ES,
    FR,
    DE
}

impl Lang 
{
    pub fn to_str(&self) -> &str 
    {
        match &self {
            Lang::EN => "EN",
            Lang::ES => "ES",
            Lang::FR => "FR",
            Lang::DE => "DE",
        }
    }

    pub fn to_string(&self) -> String 
    {
       self.to_str().to_string()
    }
}