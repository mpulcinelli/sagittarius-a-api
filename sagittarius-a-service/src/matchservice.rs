use sagittarius_a_utils::helpers::aws_helper::get_dynamo;
use sagittarius_a_utils::helpers::error_helper::LambdaGeneralError;
use sagittarius_a_utils::helpers::message_helper::{get_message, Message};
use sagittarius_a_utils::helpers::response_helper::{format_response, StatusCode};
use sagittarius_a_model::matchmodel::Match;
use aws_sdk_dynamodb::model::AttributeValue;
use serde_json::{json, Value};
use uuid::Uuid;

pub async fn add_result(game_match: Option<Match>) -> Result<Value, LambdaGeneralError<Message>> {
    let uuid = &Uuid::new_v4().to_string();
    
    let client = get_dynamo().await;

    let mat = game_match.unwrap();

    let request = client
        .put_item()
        .table_name("match")
        .item("uid", AttributeValue::S(String::from(uuid)))
        .item("score", AttributeValue::N(mat.score.to_string()))
        .item("time", AttributeValue::N(mat.time.to_string()))
        .item("data_jogo", AttributeValue::S(mat.data_jogo))
        .item(
            "user_id",
            AttributeValue::S(String::from(mat.user_id.id.to_string())),
        )
        .item(
            "game_id",
            AttributeValue::S(String::from(mat.game_id.id.to_string())),
        );

    let result = request.send().await;

    match result {
        Ok(_) => {
            let msg = get_message(vec!["00043".to_string()]).await?;
            let resp = format_response(&json!({}), StatusCode::Ok, &msg).await?;

            return Ok(resp);
        }
        Err(z) => {
            print!("{:?}", z);
            let msg = get_message(vec!["00044".to_string()]).await?;
            let resp = format_response(&json!({}), StatusCode::Forbidden, &msg).await?;

            return Ok(resp);
        }
    }
}
