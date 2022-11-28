use std::{
    env
};
use aws_sdk_dynamodb::{model::AttributeValue, Region};
use aws_sdk_dynamodb::Client;


pub fn attr_val_to_str(value: &AttributeValue) -> String {
    match value {
        AttributeValue::S(s) => s.to_string(),
        _ => unreachable!(),
    }
}


pub fn attr_val_to_bool(value: &AttributeValue) -> bool {
    match value {
        AttributeValue::Bool(s) => *s,
        _ => unreachable!(),
    }
}

pub fn attr_val_to_vec(value: &AttributeValue) -> Vec<String> {
    match value {
        AttributeValue::Ss(s) => s.to_vec(),
        _ => unreachable!(),
    }
}

pub async fn get_dynamo() -> Client {
    let dynamo_region = env::var("DYNAMO_REGION").unwrap_or_default().to_string();
    let config = aws_config::from_env().region(Region::new(dynamo_region)).load().await;
    let client = Client::new(&config);
    client
}
