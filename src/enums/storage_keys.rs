pub enum StorageKey {
    User,
    Lang,
    CameraX,
    CameraY,
    Tiles,
    ImagesDatas,
}

impl StorageKey 
{
    pub fn to_str(&self) -> &str 
    {
        match &self {
            StorageKey::User => "user",
            StorageKey::Lang => "lang",
            StorageKey::CameraX => "camera_x",
            StorageKey::CameraY => "camera_y",
            StorageKey::Tiles => "tiles",
            StorageKey::ImagesDatas => "images_datas",
        }
    }
}