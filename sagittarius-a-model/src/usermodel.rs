use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::gamemodel::Game;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserCredential {
    pub user_name: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub email: String,
    pub expires:i64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserValidation {
    pub email: String,
    pub validation_code: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserId {
    pub id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserLoginResult {
    pub id: String,
    pub user_name: String,
    pub token: String,
    pub validation_code: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserTokenValidation {
    pub id: String,
    pub user_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: String,
    pub user_name: String,

    #[serde(skip_serializing)]
    pub password: String,

    pub validation_code: String,
    pub ultimo_login: String,
    pub data_cadastro: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub habilitado: bool,
    pub perfil: Vec<String>,
    pub games: Vec<Game>,
}

impl User {
    pub fn new(
        p_id: Option<String>,
        p_user_name: Option<String>,
        p_password: Option<String>,
        p_email: Option<String>,
        p_validation_code: Option<String>,
        p_ultimo_login: Option<String>,
        p_data_cadastro: Option<String>,
        p_habilitado: Option<bool>,
        p_first_name: Option<String>,
        p_last_name: Option<String>,
        p_games: Option<Vec<Game>>,
        p_perfil: Option<Vec<String>>
    ) -> Option<Self> {
        Some(User {
            id: p_id.unwrap_or(Uuid::new_v4().to_string()),
            user_name: p_user_name.unwrap_or("".to_string()),
            password: p_password.unwrap_or("".to_string()),
            email: p_email.unwrap_or("".to_string()),
            validation_code: p_validation_code.unwrap_or("".to_string()),
            ultimo_login: p_ultimo_login.unwrap_or("".to_string()),
            data_cadastro: p_data_cadastro.unwrap_or("".to_string()),
            habilitado: p_habilitado.unwrap_or(false),
            first_name: p_first_name.unwrap_or("".to_string()),
            last_name: p_last_name.unwrap_or("".to_string()),
            games: p_games.unwrap_or(vec![]),
            perfil: p_perfil.unwrap_or(vec![]),
        })
    }
}
