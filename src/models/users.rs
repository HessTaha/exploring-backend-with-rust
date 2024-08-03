use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginData {
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserData {
    pub name: String,
    pub surname: String,
    pub id: String,
    pub age: i32,
    pub profession: String,
}
