pub enum StorageKey {
    User,
}

impl StorageKey 
{
    pub fn to_str(&self) -> &str 
    {
        match &self {
            StorageKey::User => "user",
        }
    }
}