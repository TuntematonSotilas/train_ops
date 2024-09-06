pub enum StorageKey {
    User,
    Lang,
}

impl StorageKey 
{
    pub fn to_str(&self) -> &str 
    {
        match &self {
            StorageKey::User => "user",
            StorageKey::Lang => "lang",
        }
    }
}