use hmac::{Hmac, Mac};
use ring::error::{self};

use chrono::{DateTime, Duration, Utc};
use jwt::VerifyWithKey;
use sha2::Sha384;
use std::collections::BTreeMap;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum AccessLevel {
    PLAYER,
    ADMIN,
    PLAYERVALIDATION,
    NONE,
}
pub enum ValidationType {
    NoValidations,
    ValidateUserInfo,
}

use sagittarius_a_model::usermodel::{User, UserId};

use jwt::SignWithKey;

use crate::helpers::encryption_helper::encrypt_content;

use super::encryption_helper::decrypt_content;


// pub async fn validate_token(
//     token: &String,
//     access_level: AccessLevel,
// ) -> Result<bool, error::Unspecified> {

//     let access_credential = AccessCredential::new(token);

//     println!("[SAGITTARIUS-A]=[validate_token() : ERROR: {:?}]", access_credential);

//     let access = match access_level {
//         AccessLevel::PLAYER => {
//             access_credential
//                 .access_level
//                 .contains(&AccessLevel::PLAYER)
//                 || access_credential.access_level.contains(&AccessLevel::ADMIN)
//         }
//         AccessLevel::PLAYERVALIDATION => {
//             false
//         },
//         AccessLevel::ADMIN => access_credential.access_level.contains(&AccessLevel::ADMIN),
//         AccessLevel::NONE => false,
//     };

//     if !access {
//         return Ok(false);
//     };

//     if access_credential.data_exp != "INF" {
//         let data_exp = DateTime::<Utc>::from_str(access_credential.data_exp.as_str()).unwrap();

//         if data_exp < Utc::now() {
//             return Ok(false);
//         }
//     }

//     return Ok(true);
// }



// pub async fn validate_token_checking_user(
//     token: &String,
//     user_id: &UserId,
// ) -> Result<bool, bool> {
//     if token.is_empty() {
//         return Ok(false);
//     }

//     let key: Hmac<Sha384> = Hmac::new_from_slice(b"KEY_APP_00001").unwrap();
//     let token_str = token.as_str();

//     let claims: BTreeMap<String, String> = token_str.verify_with_key(&key).unwrap_or_default();

//     if claims.len() < 3 {
//         return Err(false);
//     }

//     if !claims.contains_key("id")
//         || !claims.contains_key("user_name")
//         || !claims.contains_key("perfil")
//     {
//         return Err(false);
//     }

//     if claims["id"].is_empty() || claims["user_name"].is_empty() || claims["perfil"].is_empty() {
//         return Err(false);
//     }

//     if decrypt_content(claims["id"].clone()).unwrap() != user_id.id {
//         return Err(false);
//     }

//     // let usr = UserId {
//     //     id: claims["id"].to_string(),
//     // };

//     // let exist = id_existis(&usr).await.unwrap();

//     // if !exist {
//     //     return Ok(false);
//     // }

//     return Ok(true);
// }

pub async fn generate_user_token(
    usr: &User,
    expires: bool,
    duration_in_minutes: i64,
) -> Result<String, error::Unspecified> {
    let key: Hmac<Sha384> = Hmac::new_from_slice(b"KEY_APP_00001").unwrap();
    let mut claims = BTreeMap::new();
    
    claims.insert("id", encrypt_content(usr.id.clone()).unwrap());
    
    claims.insert("user_name", encrypt_content(usr.user_name.clone()).unwrap());
    
    let jsn_perfil = serde_json::to_string(&*usr.perfil).unwrap_or("".to_string());
    
    println!("[SAGITTARIUS-A]=[{}]", jsn_perfil);
    
    claims.insert("perfil", encrypt_content(jsn_perfil).unwrap());

    let data_cadastro_now: String;

    if !expires {
        data_cadastro_now = "INF".to_string();
        claims.insert("data_exp", data_cadastro_now);
    } else {
        data_cadastro_now = format!("{}", Utc::now() + Duration::minutes(duration_in_minutes));
        claims.insert("data_exp", data_cadastro_now);
    }

    let token_str = claims.sign_with_key(&key).unwrap();

    Ok(token_str)
}


pub async fn validate_credential(credential: &AccessCredential, access_level: AccessLevel) -> Result<bool,  error::Unspecified> {

    println!("[SAGITTARIUS-A]=[validate_token() : ERROR: {:?}]", credential);

    let access = match access_level {
        AccessLevel::PLAYER => {
            credential
                .access_level
                .contains(&AccessLevel::PLAYER)
                || credential.access_level.contains(&AccessLevel::ADMIN)
        }
        AccessLevel::PLAYERVALIDATION => {
            let is_player = credential.access_level.contains(&AccessLevel::PLAYER);
            let is_id_equal = credential.id == credential.id_to_validate;
            
            is_player && is_id_equal
        },
        AccessLevel::ADMIN => credential.access_level.contains(&AccessLevel::ADMIN),
        AccessLevel::NONE => false,
    };

    if !access {
        return Ok(false);
    };

    if credential.data_exp != "INF" {
        let data_exp = DateTime::<Utc>::from_str(credential.data_exp.as_str()).unwrap();

        if data_exp < Utc::now() {
            return Ok(false);
        }
    }

    return Ok(true);
}

#[derive(Debug)]
pub struct AccessCredential {
    pub token: String,
    pub id: String,
    pub id_to_validate:String,
    pub user_name: String,
    pub perfil: String,
    pub data_exp: String,
    pub access_level: Vec<AccessLevel>,
}

impl AccessCredential {

    pub fn set_id_to_validate(&mut self, id_to_validate:&str){
        self.id_to_validate = id_to_validate.to_string();
    }

    pub fn new(token: &String) -> AccessCredential {
        if token.is_empty() {
            AccessCredential {
                data_exp: String::new(),
                id: String::new(),
                id_to_validate:String::new(),
                perfil: String::new(),
                token: String::new(),
                user_name: String::new(),
                access_level: vec![AccessLevel::NONE],
            };
        }

        let key: Hmac<Sha384> = Hmac::new_from_slice(b"KEY_APP_00001").unwrap();
        let token_str = token.as_str();

        let claims: BTreeMap<String, String> = token_str.verify_with_key(&key).unwrap();

        if claims.len() != 4 {
            AccessCredential {
                data_exp: String::new(),
                id: String::new(),
                id_to_validate:String::new(),
                perfil: String::new(),
                token: String::new(),
                user_name: String::new(),
                access_level: vec![AccessLevel::NONE],
            };
        }

        if !claims.contains_key("id")
            || !claims.contains_key("user_name")
            || !claims.contains_key("perfil")
            || !claims.contains_key("data_exp")
        {
            AccessCredential {
                data_exp: String::new(),
                id: String::new(),
                id_to_validate:String::new(),
                perfil: String::new(),
                token: String::new(),
                user_name: String::new(),
                access_level: vec![AccessLevel::NONE],
            };
        }

        if claims["id"].is_empty()
            || claims["user_name"].is_empty()
            || claims["perfil"].is_empty()
            || claims["data_exp"].is_empty()
        {
            AccessCredential {
                data_exp: String::new(),
                id: String::new(),
                id_to_validate:String::new(),
                perfil: String::new(),
                token: String::new(),
                user_name: String::new(),
                access_level: vec![AccessLevel::NONE],
            };
        }
        
        let tmp_perfil = decrypt_content(claims["perfil"].clone()).unwrap();

        if tmp_perfil.contains("ADMIN") && tmp_perfil.contains("PLAYER"){
            return AccessCredential {
                data_exp: claims["data_exp"].to_string(),
                id: decrypt_content(claims["id"].to_string()).unwrap(),
                id_to_validate:String::new(),
                perfil: decrypt_content(claims["perfil"].to_string()).unwrap(),
                token: String::from(token_str),
                user_name: decrypt_content(claims["user_name"].to_string()).unwrap(),
                access_level: vec![AccessLevel::ADMIN, AccessLevel::PLAYER],
            };
        } else if tmp_perfil.contains("ADMIN") {
            return AccessCredential {
                data_exp: claims["data_exp"].to_string(),
                id: decrypt_content(claims["id"].to_string()).unwrap(),
                id_to_validate:String::new(),
                perfil: decrypt_content(claims["perfil"].to_string()).unwrap(),
                token: String::from(token_str),
                user_name: decrypt_content(claims["user_name"].to_string()).unwrap(),
                access_level: vec![AccessLevel::ADMIN],
            };
        } else if tmp_perfil.contains("PLAYER") {
            return AccessCredential {
                data_exp: claims["data_exp"].to_string(),
                id: decrypt_content(claims["id"].to_string()).unwrap(),
                id_to_validate:String::new(),
                perfil: decrypt_content(claims["perfil"].to_string()).unwrap(),
                token: String::from(token_str),
                user_name: decrypt_content(claims["user_name"].to_string()).unwrap(),
                access_level: vec![AccessLevel::PLAYER],
            };
        } else {
            return AccessCredential {
                data_exp: String::new(),
                id: String::new(),
                id_to_validate:String::new(),
                perfil: String::new(),
                token: String::new(),
                user_name: String::new(),
                access_level: vec![AccessLevel::NONE],
            };
        }
    }
}
