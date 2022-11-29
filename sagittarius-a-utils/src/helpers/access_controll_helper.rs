use hmac::{Hmac, Mac};
use ring::error::{self};

use chrono::{DateTime, Duration, Utc};
use jwt::VerifyWithKey;
use sha2::Sha384;
use std::collections::BTreeMap;
use std::str::FromStr;

pub enum AccessLevel {
    PLAYER,
    ADMIN,
    NONE,
}
pub enum ValidationType {
    NoValidations,
    ValidateUserInfo,
}

use sagittarius_a_model::usermodel::{User, UserId};

use jwt::SignWithKey;

pub async fn validate_token_checking_user(
    token: &String,
    user_id: &UserId,
) -> Result<bool, error::Unspecified> {
    if token.is_empty() {
        return Ok(false);
    }

    let key: Hmac<Sha384> = Hmac::new_from_slice(b"KEY_APP_00001").unwrap();
    let token_str = token.as_str();

    let claims: BTreeMap<String, String> = token_str.verify_with_key(&key).unwrap_or_default();

    if claims.len() < 3 {
        return Ok(false);
    }

    if !claims.contains_key("id")
        || !claims.contains_key("user_name")
        || !claims.contains_key("perfil")
    {
        return Ok(false);
    }

    if claims["id"].is_empty() || claims["user_name"].is_empty() || claims["perfil"].is_empty() {
        return Ok(false);
    }

    if claims["id"] != user_id.id {
        return Ok(false);
    }

    // let usr = UserId {
    //     id: claims["id"].to_string(),
    // };

    // let exist = id_existis(&usr).await.unwrap();

    // if !exist {
    //     return Ok(false);
    // }

    return Ok(true);
}

pub async fn generate_user_token(
    usr: &User,
    expires: bool,
    duration_in_minutes: i64,
) -> Result<String, error::Unspecified> {
    let key: Hmac<Sha384> = Hmac::new_from_slice(b"KEY_APP_00001").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("id", &*usr.id);
    claims.insert("user_name", &*usr.user_name);
    let jsn_perfil = serde_json::to_string(&*usr.perfil).unwrap_or("".to_string());
    claims.insert("perfil", &jsn_perfil);

    let data_cadastro_now: String;

    if !expires {
        data_cadastro_now = "INF".to_string();
        claims.insert("data_exp", &data_cadastro_now);
    } else {
        data_cadastro_now = format!("{}", Utc::now() + Duration::minutes(duration_in_minutes));
        claims.insert("data_exp", &data_cadastro_now);
    }

    let token_str = claims.sign_with_key(&key).unwrap();

    Ok(token_str)
}

pub async fn validate_token(
    token: &String,
    access_level: AccessLevel,
) -> Result<bool, error::Unspecified> {
    if token.is_empty() {
        return Ok(false);
    }

    let key: Hmac<Sha384> = Hmac::new_from_slice(b"KEY_APP_00001").unwrap();
    let token_str = token.as_str();

    let claims: BTreeMap<String, String> = token_str.verify_with_key(&key).unwrap();

    if claims.len() != 4 {
        return Ok(false);
    }

    if !claims.contains_key("id")
        || !claims.contains_key("user_name")
        || !claims.contains_key("perfil")
        || !claims.contains_key("data_exp")
    {
        return Ok(false);
    }

    if claims["id"].is_empty()
        || claims["user_name"].is_empty()
        || claims["perfil"].is_empty()
        || claims["data_exp"].is_empty()
    {
        return Ok(false);
    }

    println!("[SAGITTARIUS-A]={}]", claims["perfil"]);

    let access = match access_level {
        AccessLevel::PLAYER => {
            if claims["perfil"].contains("PLAYER") || claims["perfil"].contains("ADMIN") {
                true
            } else {
                false
            }
        }
        AccessLevel::ADMIN => {
            if claims["perfil"].contains("ADMIN") {
                true
            } else {
                false
            }
        }
        AccessLevel::NONE => false,
    };

    if !access {
        return Ok(false);
    };

    if claims["data_exp"] != "INF" {
        let data_exp = DateTime::<Utc>::from_str(claims["data_exp"].as_str()).unwrap();

        if data_exp < Utc::now() {
            return Ok(false);
        }
    }

    return Ok(true);
}

pub async fn get_user_profile(token: &String) -> Result<Vec<AccessLevel>, error::Unspecified> {
    if token.is_empty() {
        return Ok(vec![AccessLevel::NONE]);
    }

    let key: Hmac<Sha384> = Hmac::new_from_slice(b"KEY_APP_00001").unwrap();
    let token_str = token.as_str();

    let claims: BTreeMap<String, String> = token_str.verify_with_key(&key).unwrap();

    if claims.len() != 4 {
        return Ok(vec![AccessLevel::NONE]);
    }

    if !claims.contains_key("id")
        || !claims.contains_key("user_name")
        || !claims.contains_key("perfil")
        || !claims.contains_key("data_exp")
    {
        return Ok(vec![AccessLevel::NONE]);
    }

    if claims["id"].is_empty()
        || claims["user_name"].is_empty()
        || claims["perfil"].is_empty()
        || claims["data_exp"].is_empty()
    {
        return Ok(vec![AccessLevel::NONE]);
    }

    if claims["perfil"].contains("PLAYER") {
        return Ok(vec![AccessLevel::PLAYER]);
    } else if claims["perfil"].contains("ADMIN") {
        return Ok(vec![AccessLevel::ADMIN, AccessLevel::PLAYER]);
    }else{
        return Ok(vec![AccessLevel::NONE]);
    }

    
}
