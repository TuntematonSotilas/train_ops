use crate::services::storage;

use super::storage_keys::StorageKey;

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

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String 
    {
       self.to_str().to_string()
    }

    pub fn from_storage() -> Option<Self> {
        if let Some(lang_str) = storage::get(StorageKey::Lang) {
            let lang = Self::from_str(lang_str.as_str());
            return Some(lang)
        }
        None
    }

    fn from_str(str_lang: &str) -> Self 
    {
        match str_lang {
           "ES" =>  Lang::ES,
           "FR" => Lang::FR,
           "DE" => Lang::DE,
           _ =>  Lang::EN,
        }
    }
}