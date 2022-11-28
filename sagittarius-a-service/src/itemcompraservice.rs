use sagittarius_a_model::gamemodel::GameId;
use sagittarius_a_model::itemcompramodel::{ItemCompra, ItemCompraSKU};
use sagittarius_a_model::usermodel::UserId;
use sagittarius_a_utils::helpers::aws_helper::{attr_val_to_bool, attr_val_to_str, get_dynamo};
use sagittarius_a_utils::helpers::error_helper::LambdaGeneralError;
use sagittarius_a_utils::helpers::message_helper::{get_message, Message};
use sagittarius_a_utils::helpers::response_helper::{format_response, StatusCode};

use aws_sdk_dynamodb::model::AttributeValue;
use serde_json::{json, Value};
use uuid::Uuid;

pub async fn add_item(
    item_compra: Option<ItemCompra>,
) -> Result<Value, LambdaGeneralError<Message>> {
    let uuid = &Uuid::new_v4().to_string();

    let client = get_dynamo().await;

    let item = item_compra.unwrap();

    let qtd = count_item_from_user(&item).await?;

    if qtd > 0 {
        let msg = get_message(vec!["00059".to_string()]).await?;
        let resp = format_response(&json!({}), StatusCode::Forbidden, &msg).await?;
        return Ok(resp);
    }

    let request = client
        .put_item()
        .table_name("item_compra")
        .item("uid", AttributeValue::S(String::from(uuid)))
        .item("sku", AttributeValue::S(String::from(item.sku.to_string())))
        .item(
            "user_id",
            AttributeValue::S(String::from(item.user_id.id.to_string())),
        )
        .item(
            "game_id",
            AttributeValue::S(String::from(item.game_id.id.to_string())),
        );

    let result = request.send().await;

    match result {
        Ok(_) => {
            let msg = get_message(vec!["00057".to_string()]).await?;
            let resp = format_response(&json!({}), StatusCode::Ok, &msg).await?;

            return Ok(resp);
        }
        Err(z) => {
            print!("{:?}", z);
            let msg = get_message(vec!["00058".to_string()]).await?;
            let resp = format_response(&json!({}), StatusCode::Forbidden, &msg).await?;

            return Ok(resp);
        }
    }
}

pub async fn count_item_from_user(item: &ItemCompra) -> Result<usize, LambdaGeneralError<Message>> {
    let client = get_dynamo().await;

    let resp = client
        .scan()
        .filter_expression("sku = :sku and user_id = :user_id")
        .expression_attribute_values(":sku", AttributeValue::S(item.sku.to_string()))
        .expression_attribute_values(":user_id", AttributeValue::S(item.user_id.id.to_string()))
        .table_name("item_compra")
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

pub enum ReturnItemStatus {

    ReleasedForPlayer,
    NotReleasedForPlayer,
    AllItems

}


pub async fn update_released_for_player(item: &ItemCompra) -> Result<bool, LambdaGeneralError<Message>> {
    let client = get_dynamo().await;

    let request = client
        .update_item()
        .table_name("item_compra")
        .key("uid", AttributeValue::S(item.id.to_string()))
        .update_expression("set is_released_for_player = :is_released_for_player")
        .expression_attribute_values(":is_released_for_player", AttributeValue::Bool(true));

    let result = request.send().await;
    match result {
        Ok(_) => {
            return Ok(true);
        }
        Err(z) => {
            print!("{:?}", z);
            return Ok(false);
        }
    }
}

pub async fn list_items_from_sku(
    item: &ItemCompraSKU,
    return_status: ReturnItemStatus
) -> Result<Vec<ItemCompra>, LambdaGeneralError<Message>> {
    let client = get_dynamo().await;

    let itm = item;

       let resp = match return_status{
            ReturnItemStatus::AllItems=>{
                client
                .scan()
                .filter_expression("sku = :sku")
                .expression_attribute_values(":sku", AttributeValue::S(itm.sku.to_string()))
                .table_name("item_compra")
                .send()
                .await                
            }
            ReturnItemStatus::ReleasedForPlayer => {
                client
                .scan()
                .filter_expression("sku = :sku and is_released_for_player = :is_released_for_player")
                .expression_attribute_values(":sku", AttributeValue::S(itm.sku.to_string()))
                .expression_attribute_values(":is_released_for_player", AttributeValue::Bool(true))
                .table_name("item_compra")
                .send()
                .await
            },
            ReturnItemStatus::NotReleasedForPlayer => {
                client
                .scan()
                .filter_expression("sku = :sku and is_released_for_player = :is_released_for_player")
                .expression_attribute_values(":sku", AttributeValue::S(itm.sku.to_string()))
                .expression_attribute_values(":is_released_for_player", AttributeValue::Bool(false))
                .table_name("item_compra")
                .send()
                .await                
            },
        };


    match resp {
        Ok(r) => {
            let result: Vec<ItemCompra> = r
                .items
                .unwrap()
                .into_iter()
                .map(|f| ItemCompra {
                    id: attr_val_to_str(f.get("uid").unwrap()).to_string(),
                    sku: attr_val_to_str(f.get("sku").unwrap()).to_string(),
                    user_id: UserId {
                        id: attr_val_to_str(f.get("user_id").unwrap()).to_string(),
                    },
                    is_released_for_player: attr_val_to_bool(
                        f.get("is_released_for_player").unwrap(),
                    ),
                    game_id: GameId {
                        id: attr_val_to_str(f.get("game_id").unwrap()).to_string(),
                    },
                })
                .collect();

            Ok(result)
        }
        Err(z) => {
            print!("{:?}", z);
            let msg = get_message(vec!["00061".to_string()]).await?;
            return Err(LambdaGeneralError { messages: msg });
        }
    }
}
