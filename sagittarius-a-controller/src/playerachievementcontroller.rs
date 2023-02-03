use chrono::Utc;
use serde_json::{json, Value};

use sagittarius_a_model::{
    gamemodel::GameId, playerachievement::PlayerAchievement, usermodel::UserId,
};
use sagittarius_a_utils::helpers::{
    access_controll_helper::{ AccessLevel, AccessCredential, validate_credential},
    error_helper::LambdaGeneralError,
    message_helper::{get_message, Message},
    response_helper::{format_response, StatusCode},
};

use sagittarius_a_service::{
    playerachievementservice::{add_achievement, verify_item_achieved},
    userservice::id_existis,
};

pub async fn ctrl_verify_achievement(event: &Value) -> Result<Value, LambdaGeneralError<Message>> {
    let token = event["token"].as_str().unwrap_or("").to_string();
    let usr_id = UserId {
        id: event["user_id"].as_str().unwrap_or("0").to_string(),
    };

    let exist = id_existis(&usr_id).await.unwrap_or(false);
    
    let mut access = AccessCredential::new(&token);
    access.set_id_to_validate(&usr_id.id);

    if !validate_credential(&access, AccessLevel::PLAYERVALIDATION)
        .await
        .unwrap_or(false) || !exist
    {
        let msg = get_message(vec!["00056".to_string(), "00045".to_string()]).await?;
        let resp = format_response(&json!({}), StatusCode::Forbidden, &msg).await?;
        return Ok(resp);
    }

    let now = Utc::now();
    let data_cadastro_now: String = format!("{}", now);

    let new_achievement = PlayerAchievement {
        id: "".to_string(),
        game_id: GameId {
            id: event["game_id"].as_str().unwrap_or("0").to_string(),
        },
        user_id: usr_id,
        tipo: event["tipo"].as_str().unwrap_or("NULL").to_string(),
        timestamp: data_cadastro_now,
        item: event["achievement_item"]
            .as_str()
            .unwrap_or("0")
            .to_string(),
    };

    let result = verify_item_achieved(&new_achievement).await?;

    let msg = get_message(vec!["00066".to_string()]).await?;
    let resp = format_response(&json!({ "len": result }), StatusCode::Ok, &msg).await?;

    return Ok(resp);
}

pub async fn ctrl_register_new_player_achievement(
    event: &Value,
) -> Result<Value, LambdaGeneralError<Message>> {
    let token = event["token"].as_str().unwrap_or("").to_string();
    let usr_id = UserId {
        id: event["user_id"].as_str().unwrap_or("0").to_string(),
    };

    let exist = id_existis(&usr_id).await.unwrap_or(false);
    
    let mut access = AccessCredential::new(&token);
    access.set_id_to_validate(&usr_id.id);

    if !validate_credential(&access, AccessLevel::PLAYERVALIDATION)
        .await
        .unwrap_or(false) || !exist
    {
        let msg = get_message(vec!["00056".to_string(), "00045".to_string()]).await?;
        let resp = format_response(&json!({}), StatusCode::Forbidden, &msg).await?;
        return Ok(resp);
    }

    let now = Utc::now();
    let data_cadastro_now: String = format!("{}", now);

    let new_achievement = Some(PlayerAchievement {
        id: "".to_string(),
        game_id: GameId {
            id: event["game_id"].as_str().unwrap_or("0").to_string(),
        },
        user_id: usr_id,
        tipo: event["tipo"].as_str().unwrap_or("NULL").to_string(),
        timestamp: data_cadastro_now,
        item: event["achievement_item"]
            .as_str()
            .unwrap_or("0")
            .to_string(),
    });

    let result = add_achievement(new_achievement).await?;

    Ok(result)
}
