use chrono::Utc;
use serde_json::{json, Value};


use sagittarius_a_utils::helpers::{
    error_helper::LambdaGeneralError,
    message_helper::{get_message, Message},
    response_helper::{format_response, StatusCode},
    access_controll_helper::{AccessLevel, AccessCredential, validate_credential},

};
use sagittarius_a_model::{
    gamemodel::GameId, matchmodel::Match, usermodel::UserId
};

use sagittarius_a_service::{
    matchservice::add_result, userservice::id_existis
};


pub async fn ctrl_register_match_result(
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
        let msg = get_message(vec!["00044".to_string(), "00045".to_string()]).await?;
        let resp = format_response(&json!({}), StatusCode::Forbidden, &msg).await?;
        return Ok(resp);
    }

    let now = Utc::now();
    let data_cadastro_now: String = format!("{}", now);

    let new_match = Some(Match {
        id: "".to_string(),
        data_jogo: data_cadastro_now,
        score: event["score"]
            .as_str()
            .unwrap_or("0")
            .to_string()
            .parse::<i32>()
            .unwrap_or(0),
        time: event["time"]
            .as_str()
            .unwrap_or("0")
            .to_string()
            .parse::<i32>()
            .unwrap_or(0),
        game_id: GameId {
            id: event["game_id"].as_str().unwrap_or("0").to_string(),
        },
        user_id: usr_id,
    });

    let result = add_result(new_match).await?;

    Ok(result)
}
