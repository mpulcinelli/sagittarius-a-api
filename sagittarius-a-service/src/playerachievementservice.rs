use sagittarius_a_utils::helpers::aws_helper::get_dynamo;
use sagittarius_a_utils::helpers::error_helper::LambdaGeneralError;
use sagittarius_a_utils::helpers::message_helper::{get_message, Message};
use sagittarius_a_utils::helpers::response_helper::{format_response, StatusCode};
use sagittarius_a_model::playerachievement::PlayerAchievement;
use aws_sdk_dynamodb::types::AttributeValue;
use serde_json::{json, Value};
use uuid::Uuid;


pub async fn verify_item_achieved(item: &PlayerAchievement) -> Result<usize, LambdaGeneralError<Message>> {
    let client = get_dynamo().await;

    let resp = client
        .scan()
        .filter_expression("achievement_item = :achievement_item and game_id = :game_id and tipo = :tipo")
        .expression_attribute_values(":achievement_item", AttributeValue::S(item.item.to_string()))
        .expression_attribute_values(":game_id", AttributeValue::S(item.game_id.id.to_string()))
        .expression_attribute_values(":tipo", AttributeValue::S(item.tipo.to_string()))
        .table_name("player_achievement")
        .send()
        .await;

    match resp {
        Ok(r) => {
            let itms = r.items.unwrap().len();
            Ok(itms)
        }
        Err(z) => {
            print!("{:?}", z);
            let msg = get_message(vec!["00060".to_string()]).await?;
            return Err(LambdaGeneralError { messages: msg });
        }
    }
}



pub async fn add_achievement(
    player_achievement: Option<PlayerAchievement>,
) -> Result<Value, LambdaGeneralError<Message>> {
    let uuid = &Uuid::new_v4().to_string();
    
    let client = get_dynamo().await;

    let item = player_achievement.unwrap();

    let qtd = verify_item_achieved(&item).await?;//count_item_from_user(&item).await?;

    if qtd > 0 {
        let msg = get_message(vec!["00065".to_string()]).await?;
        let resp = format_response(&json!({}), StatusCode::Forbidden, &msg).await?;

        return Ok(resp);
    }

    let request = client
        .put_item()
        .table_name("player_achievement")
        .item("uid", AttributeValue::S(String::from(uuid)))
        .item("tipo", AttributeValue::S(String::from(item.tipo.to_string())))
        .item("achievement_item", AttributeValue::S(String::from(item.item.to_string())))
        .item(
            "user_id",
            AttributeValue::S(String::from(item.user_id.id.to_string())),
        )
        .item(
            "game_id",
            AttributeValue::S(String::from(item.game_id.id.to_string())),
        )
        .item("timestamp", AttributeValue::S(item.timestamp.to_string()))
        ;

    let result = request.send().await;

    match result {
        Ok(_) => {
            let msg = get_message(vec!["00063".to_string()]).await?;
            let resp = format_response(&json!({}), StatusCode::Ok, &msg).await?;

            return Ok(resp);
        }
        Err(z) => {
            print!("{:?}", z);
            let msg = get_message(vec!["00064".to_string()]).await?;
            let resp = format_response(&json!({}), StatusCode::Forbidden, &msg).await?;

            return Ok(resp);
        }
    }
}