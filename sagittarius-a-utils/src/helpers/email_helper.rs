use crate::helpers::error_helper::LambdaGeneralError;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sesv2::model::{Body, Content, Destination, EmailContent, Message};
use aws_sdk_sesv2::Client;

use super::message_helper::get_message;

pub async fn send_email(
    from: &str,
    to: &str,
    subject: &str,
    message: &str,
) -> Result<String, LambdaGeneralError<super::message_helper::Message>> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let dest = Destination::builder().to_addresses(to).build();
    let subject_content = Content::builder().data(subject).charset("UTF-8").build();
    let body_content = Content::builder().data(message).charset("UTF-8").build();
    let body = Body::builder().html(body_content).build();

    let msg = Message::builder()
        .subject(subject_content)
        .body(body)
        .build();

    let email_content = EmailContent::builder().simple(msg).build();

    let res = client
        .send_email()
        .from_email_address(from)
        .destination(dest)
        .content(email_content)
        .send()
        .await;

    //Ok(res.message_id.unwrap())
    match res.unwrap().message_id {
        Some(a) => Ok(a),
        None => Err(LambdaGeneralError {
            messages: get_message(vec!["00047".to_string()]).await.unwrap(),
        }),
    }
}
