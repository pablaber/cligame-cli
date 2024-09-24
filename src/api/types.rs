use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ApiUser {
    pub id: String,
    pub email: String,
    pub character: ApiCharacter,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ApiCharacter {
    pub name: String,
    pub money: u32,
}
