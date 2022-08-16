use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct RegisterRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct RegisterResponse {
    pub success: bool,
}