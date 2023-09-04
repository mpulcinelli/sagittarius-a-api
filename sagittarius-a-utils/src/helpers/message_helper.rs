use super::{error_helper::LambdaGeneralError, aws_helper::attr_val_to_str};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
//use std::sync::Mutex;

//thread_local!(static ref API_LANG: Mutex<String> = Mutex::new(String::from("")));
thread_local!(static API_LANG: RefCell<String> = RefCell::new("".to_string()));

// lazy_static! {
//     ;
// }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub lang: String,
    pub id: String,
    pub mensagem: String,
}

pub fn set_lang(lang: String) {
    API_LANG.with(|text| {
        *text.borrow_mut() = lang;
    });
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DictFieldVal {
    pub field: String,
    pub value: String,
}

pub async fn get_message_with_fields(
    id: String,
    fields: &Vec<DictFieldVal>,
) -> Result<String, LambdaGeneralError<Message>> {
    let msg = get_message(vec![id]).await?;

    let res: &Message = &msg[0];

    let mut m = res.mensagem.to_string();

    for f in fields {
        m = m.replace(f.field.as_str(), f.value.as_str());
    }

    Ok(m)
}

pub async fn get_message(ids: Vec<String>) -> Result<Vec<Message>, LambdaGeneralError<Message>> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);
    let mut lang = "".to_string();

    API_LANG.with(|text| {
        lang = format!("{}", *text.borrow());
    });

    println!("[SAGITTARIUS-A]=[get_message] lang: {}", lang);

    let mut expr = "".to_string();
    let mut resp = client
        .scan()
        .table_name("messages")
        .expression_attribute_values(":langval", AttributeValue::S(lang.to_string()));
    let mut c = "".to_string();

    for i in ids {
        let item = &format!(":p{}", i);
        expr = [c, item.to_string(), expr].join(" ");

        resp = resp.expression_attribute_values(item, AttributeValue::S(i));
        c = ",".to_string();
    }

    let complete_expre = [
        "lang = :langval and id IN (".to_string(),
        expr,
        ")".to_string(),
    ]
    .join("");

    let newresp = resp
        .filter_expression(complete_expre)
        .table_name("messages")
        .send()
        .await;

    match newresp {
        Ok(r) => {
            let result: Vec<Message> = r
                .items
                .unwrap()
                .into_iter()
                .map(|f| Message {
                    id: attr_val_to_str(f.get("id").unwrap()).to_string(),
                    lang: attr_val_to_str(f.get("lang").unwrap()).to_string(),
                    mensagem: attr_val_to_str(f.get("mensagem").unwrap()).to_string(),
                })
                .collect();

            Ok(result)
        }
        Err(e) => {
            println!("[SAGITTARIUS-A]=[get_message] error: {}", e);
            Err(LambdaGeneralError {
                messages: vec![Message {
                    id: "-10000".to_string(),
                    lang: "en-us".to_string(),
                    mensagem: "Internal problem occurred".to_string(),
                }],
            })
        }
    }
}
