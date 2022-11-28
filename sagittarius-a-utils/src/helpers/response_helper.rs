use crate::helpers::message_helper::Message;
use serde::{Deserialize, Serialize};
//use serde_json::Result;
use serde_json::{json, Value};

use super::error_helper::LambdaGeneralError;
use super::message_helper::get_message;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    NotAcceptable = 406,
}

pub async fn format_response(
    body: &Value,
    status: StatusCode,
    message: &Vec<Message>,
) -> Result<Value, LambdaGeneralError<Message>> {
    if message.len() > 0 {
        let value_respose = json!({
            "status":[
                {"code":status.clone() as i32, "name":status}
            ],
            "message": message,
            "body": body
        });

        return Ok(value_respose);
    } else {
        return Err(LambdaGeneralError {
            messages: get_message(vec!["00048".to_string()]).await.unwrap(),
        });
    }
}
