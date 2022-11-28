use chrono::Utc;
use serde_json::{json, Value};
use uuid::Uuid;


use sagittarius_a_utils::helpers::{
    error_helper::LambdaGeneralError,
    jwt_helper::validate_token,
    message_helper::{get_message, Message},
    response_helper::{format_response, StatusCode},
};
use sagittarius_a_model::{
    gamemodel::GameId,
    usermodel::{User, UserCredential, UserId, UserValidation}    
};

use sagittarius_a_service::{
    userservice::{
        add_new_user, assign_game, do_login, do_recover_password_f1, do_recover_password_f2,
        do_recover_password_invalid, get_all, get_validation_code, remove_user, validate_user_code,
    }    
};

pub async fn ctrl_get_all(event: &Value) -> Result<Value, LambdaGeneralError<Message>> {
    let tkn = event["token"].as_str().unwrap_or("").to_string();

    if !validate_token(&tkn).await.unwrap_or(false) {
        let msg = get_message(vec!["00022".to_string()]).await?;
        let r = format_response(&json!({}), StatusCode::BadRequest, &msg).await?;
        return Ok(r);
    }

    let list = get_all().await?;

    Ok(list)
}

pub async fn ctrl_add_new_user(event: &Value) -> Result<Value, LambdaGeneralError<Message>> {
    let now = Utc::now();
    let data_cadastro_now: String = format!("{}", now);
    let valida_codigo = &Uuid::new_v4().to_string();

    let new_user = Some(User {
        data_cadastro: data_cadastro_now,
        email: event["email"].as_str().unwrap_or("").to_string(),
        first_name: event["first_name"].as_str().unwrap_or("").to_string(),
        habilitado: event["xsolla"].as_bool().unwrap_or(false),
        id: "".to_string(),
        last_name: event["last_name"].as_str().unwrap_or("").to_string(),
        password: event["password"].as_str().unwrap_or("").to_string(),
        ultimo_login: "".to_string(),
        user_name: event["user_name"].as_str().unwrap_or("").to_string(),
        validation_code: valida_codigo.to_string(),
        games: vec![],
        perfil: vec!["PLAYER".to_string()],
    });

    let result = add_new_user(new_user).await.unwrap();

    Ok(result)
}

pub async fn ctrl_remove_user(event: &Value) -> Result<Value, LambdaGeneralError<Message>> {
    let uid = event["uid"].as_str().unwrap_or("");

    let user_to_delete = Some(UserId {
        id: uid.to_string(),
    });

    let result = remove_user(user_to_delete).await.unwrap();

    Ok(result)
}

pub async fn ctrl_do_login_user(event: &Value) -> Result<Value, LambdaGeneralError<Message>> {
    let user_credentiasl = UserCredential {
        email: event["email"].as_str().unwrap_or("").to_string(),
        password: event["password"].as_str().unwrap_or("").to_string(),
        user_name: event["user_name"].as_str().unwrap_or("").to_string(),
        expires: event["expires"].as_i64().unwrap_or(5),
    };

    let result = do_login(&user_credentiasl).await.unwrap();

    Ok(result)
}

pub async fn ctrl_recover_user_password(
    event: &Value,
) -> Result<Value, LambdaGeneralError<Message>> {
    let fase = event["f"].as_str().unwrap_or("").to_string();

    if fase == "F1" {
        let new_user = UserCredential {
            email: event["email"].as_str().unwrap_or("").to_string(),
            password: "".to_string(),
            user_name: "".to_string(),
            expires: 0,
        };

        let result = do_recover_password_f1(&new_user).await?;

        return Ok(result);
    } else if fase == "F2" {
        let new_user = UserCredential {
            email: event["email"].as_str().unwrap_or("").to_string(),
            password: event["password"].as_str().unwrap_or("").to_string(),
            user_name: event["user_name"].as_str().unwrap_or("").to_string(),
            expires: 0,
        };

        let u_val = UserValidation {
            email: event["email"].as_str().unwrap_or("").to_string(),
            validation_code: event["validation_code"].as_str().unwrap_or("").to_string(),
        };

        let result = do_recover_password_f2(&new_user, &u_val).await?;
        return Ok(result);
    } else {
        let result = do_recover_password_invalid().await?;
        return Ok(result);
    }
}

pub async fn ctrl_validate_register_code(
    event: &Value,
) -> Result<Value, LambdaGeneralError<Message>> {
    let new_user = UserValidation {
        validation_code: event["validation_code"].as_str().unwrap_or("").to_string(),
        email: event["email"].as_str().unwrap_or("").to_string(),
    };

    let result = validate_user_code(&new_user).await?;

    Ok(result)
}

pub async fn ctrl_assign_game_to_user(event: &Value) -> Result<Value, LambdaGeneralError<Message>> {
    let user_id = UserId {
        id: event["user_id"].as_str().unwrap_or("").to_string(),
    };

    let game_id = GameId {
        id: event["game_id"].as_str().unwrap_or("").to_string(),
    };

    let result = assign_game(&user_id, &game_id).await?;

    Ok(result)
}

pub async fn ctrl_get_user_validation_code(
    event: &Value,
) -> Result<Value, LambdaGeneralError<Message>> {
    let new_user = UserCredential {
        password: event["password"].as_str().unwrap_or("").to_string(),
        user_name: event["user_name"].as_str().unwrap_or("").to_string(),
        email: event["email"].as_str().unwrap_or("").to_string(),
        expires: 0,
    };

    let result = get_validation_code(&new_user).await?;

    Ok(result)
}
