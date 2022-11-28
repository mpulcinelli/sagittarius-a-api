use aws_sdk_dynamodb::model::AttributeValue;
use aws_config::meta::region::RegionProviderChain;
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
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);
    client
}
