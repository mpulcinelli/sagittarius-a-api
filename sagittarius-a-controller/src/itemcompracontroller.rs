//use chrono::Utc;
use serde_json::{json, Value};

use sagittarius_a_model::{
    gamemodel::GameId,
    itemcompramodel::{ItemCompra, ItemCompraSKU},
    usermodel::UserId,
};
use sagittarius_a_utils::helpers::{
    email_helper,
    error_helper::LambdaGeneralError,
    jwt_helper::{validate_token, validate_token_checking_user},
    message_helper::{get_message, Message},
    response_helper::{format_response, StatusCode},
};

use sagittarius_a_service::{
    itemcompraservice::{
        add_item, list_items_from_sku, update_released_for_player, ReturnItemStatus,
    },
    userservice::get_from_id,
};

pub async fn ctrl_register_new_item(event: &Value) -> Result<Value, LambdaGeneralError<Message>> {
    let token = event["token"].as_str().unwrap_or("").to_string();
    let usr_id = UserId {
        id: event["user_id"].as_str().unwrap_or("0").to_string(),
    };

    if !validate_token_checking_user(&token, &usr_id)
        .await
        .unwrap_or(false)
    {
        let msg = get_message(vec!["00056".to_string(), "00045".to_string()]).await?;
        let resp = format_response(&json!({}), StatusCode::Forbidden, &msg).await?;
        return Ok(resp);
    }

    // let now = Utc::now();
    // let data_cadastro_now: String = format!("{}", now);

    let new_item = Some(ItemCompra {
        id: "".to_string(),
        is_released_for_player: false,
        sku: event["sku"].as_str().unwrap_or("0").to_string(),
        game_id: GameId {
            id: event["game_id"].as_str().unwrap_or("0").to_string(),
        },
        user_id: usr_id,
    });

    let result = add_item(new_item).await?;

    Ok(result)
}

pub async fn ctrl_get_all_items_from_sku(
    event: &Value,
) -> Result<Value, LambdaGeneralError<Message>> {
    let tkn = event["token"].as_str().unwrap_or("").to_string();
    let count = event["count"].as_str().unwrap_or("0").to_string();

    if !validate_token(&tkn).await.unwrap_or(false) {
        let msg = get_message(vec!["00022".to_string()]).await?;
        let r = format_response(&json!({}), StatusCode::BadRequest, &msg).await?;
        return Ok(r);
    }
    let item = ItemCompraSKU {
        sku: event["sku"].as_str().unwrap_or("0").to_string(),
    };

    let list = list_items_from_sku(&item, ReturnItemStatus::AllItems).await?;

    if count == "1" {
        let itemlen = list.len() as i32;
        let msg = get_message(vec!["00062".to_string()]).await?;
        let r = format_response(&json!({ "sum": itemlen }), StatusCode::Ok, &msg).await?;
        return Ok(r);
    }

    let msg = get_message(vec!["00062".to_string()]).await?;
    let r = format_response(&json!(list), StatusCode::Ok, &msg).await?;
    return Ok(r);
}

pub async fn ctrl_notify_users_item_compra_sku(
    event: &Value,
) -> Result<Value, LambdaGeneralError<Message>> {
    let tkn = event["token"].as_str().unwrap_or("").to_string();
    let description = event["description"].as_str().unwrap_or("").to_string();
    
    if !validate_token(&tkn).await.unwrap_or(false) {
        let msg = get_message(vec!["00022".to_string()]).await?;
        let r = format_response(&json!({}), StatusCode::BadRequest, &msg).await?;
        return Ok(r);
    }

    let item = ItemCompraSKU {
        sku: event["sku"].as_str().unwrap_or("0").to_string(),
    };

    let list = list_items_from_sku(&item, ReturnItemStatus::NotReleasedForPlayer).await?;

    for s in &list {
        let curr_user = get_from_id(&s.user_id).await?;

        if !curr_user.is_empty() {
            let is_ok = update_released_for_player(&s).await?;
            if is_ok {
                email_helper::send_email(
                    "mpulcinelli@gmail.com",
                    curr_user[0].email.as_str(),
                    "JOGO LIBERADO PARA SER JOGADO",
                    description.as_str(),
                )
                .await?;
            }
        }
    }
    
    if !list.is_empty() {
        let msg = get_message(vec!["00067".to_string()]).await?;
        let r = format_response(&json!({}), StatusCode::Ok, &msg).await?;

        return Ok(r);
    } else {
        let msg = get_message(vec!["00068".to_string()]).await?;
        let r = format_response(&json!({}), StatusCode::Ok, &msg).await?;

        return Ok(r);

    }
}
