use sagittarius_a_utils::helpers::aws_helper::{attr_val_to_bool, attr_val_to_str, attr_val_to_vec, get_dynamo};
use sagittarius_a_utils::helpers::email_helper::send_email;
use sagittarius_a_utils::helpers::encryption_helper::encrypt;
use sagittarius_a_utils::helpers::error_helper::LambdaGeneralError;
use sagittarius_a_utils::helpers::access_controll_helper::generate_user_token;
use sagittarius_a_utils::helpers::message_helper::{get_message, get_message_with_fields, DictFieldVal, Message};
use sagittarius_a_utils::helpers::response_helper::{format_response, StatusCode};
use sagittarius_a_model::gamemodel::GameId;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::Utc;
use serde_json::json;
use serde_json::Value;

use uuid::Uuid;

use sagittarius_a_model::usermodel::{User, UserCredential, UserId, UserLoginResult, UserValidation};

pub async fn add_new_user(user: Option<User>) -> Result<Value, LambdaGeneralError<Message>> {
    let uuid = &Uuid::new_v4().to_string();
    let client = get_dynamo().await;

    let usr = user.unwrap();

    if usr.email.trim().eq("") {
        let msg = get_message(vec!["00008".to_string()]).await?;
        let r = format_response(&json!({}), StatusCode::NotFound, &msg).await?;
        return Ok(r);
    }
    if usr.user_name.trim().eq("") {
        let msg = get_message(vec!["00019".to_string()]).await?;
        let r = format_response(&json!({}), StatusCode::NotFound, &msg).await?;
        return Ok(r);
    }
    if usr.password.trim().eq("") {
        let msg = get_message(vec!["00009".to_string()]).await?;
        let r = format_response(&json!({}), StatusCode::NotFound, &msg).await?;
        return Ok(r);
    }
    if usr.password.len() < 5 {
        let msg = get_message(vec!["00010".to_string()]).await?;
        let r = format_response(&json!({}), StatusCode::NotAcceptable, &msg).await?;
        return Ok(r);
    }

    let exist = email_existis(&usr).await?;

    if exist {
        let msg = get_message(vec!["00034".to_string()]).await?;
        let r = format_response(&json!({}), StatusCode::NotAcceptable, &msg).await?;
        return Ok(r);
    }

    let exist = login_existis(&usr).await;

    match exist {
        Ok(true) => {
            let msg = get_message(vec!["00035".to_string()]).await?;
            let r = format_response(&json!({}), StatusCode::NotAcceptable, &msg).await?;
            return Ok(r);
        }
        Ok(false) => {}
        Err(z) => {
            print!("{:?}", z);
            let msg = get_message(vec!["00036".to_string()]).await?;
            return Err(LambdaGeneralError { messages: msg });
        }
    }

    let request = client
        .put_item()
        .table_name("user")
        .item("uid", AttributeValue::S(String::from(uuid)))
        .item(
            "user_name",
            AttributeValue::S(String::from(usr.user_name.to_string())),
        )
        .item(
            "password",
            AttributeValue::S(String::from(encrypt(usr.password.to_string()).unwrap())),
        )
        .item(
            "email",
            AttributeValue::S(String::from(usr.email.to_string())),
        )
        .item(
            "validation_code",
            AttributeValue::S(String::from(usr.validation_code.to_string())),
        )
        .item(
            "ultimo_login",
            AttributeValue::S(String::from(usr.ultimo_login.to_string())),
        )
        .item(
            "data_cadastro",
            AttributeValue::S(String::from(usr.data_cadastro.to_string())),
        )
        .item("habilitado", AttributeValue::Bool(usr.habilitado))
        .item("first_name", AttributeValue::S(usr.first_name.to_string()))
        .item(
            "last_name",
            AttributeValue::S(String::from(usr.last_name.to_string())),
        )
        .item("perfil", AttributeValue::Ss(usr.perfil.to_vec()));

    println!("[SAGITTARIUS-A]=[add_new_user :: {:?}]", &usr);

    let result = request.send().await;

    match result {
        Ok(_) => {
            // Usuário estará habilitado somente se vier da XSOLLA.
            if usr.habilitado == false {
                let items = vec![
                    DictFieldVal {
                        field: String::from("@userName@"),
                        value: usr.user_name,
                    },
                    DictFieldVal {
                        field: String::from("@generatedCode@"),
                        value: usr.validation_code,
                    },
                ];

                let conteudo = get_message_with_fields("00018".to_string(), &items).await?;

                println!("[SAGITTARIUS-A]=[add_new_user :: {:?}]", conteudo);

                let subject = get_message(vec!["00017".to_string()]).await?;

                send_email(
                    "mp@unrealbrasil.com.br",
                    usr.email.as_str(),
                    subject[0].mensagem.as_str(),
                    conteudo.as_str(),
                )
                .await?;
            }

            let msg = get_message(vec!["00011".to_string()]).await?;
            let r = format_response(&json!({}), StatusCode::Ok, &msg).await?;

            return Ok(r);
        }
        Err(z) => {
            print!("{:?}", z);
            let msg = get_message(vec!["00012".to_string()]).await?;
            return Err(LambdaGeneralError { messages: msg });
        }
    }
}

// TODO: testar o método remover usuário.
pub async fn remove_user(user: Option<UserId>) -> Result<Value, LambdaGeneralError<Message>> {
    let client = get_dynamo().await;

    let usr = user.unwrap();

    let uuld = usr.id;

    if uuld.trim().eq("") {
        let msg = get_message(vec!["00022".to_string()]).await?;
        let r = format_response(&json!({}), StatusCode::NotFound, &msg).await?;
        return Ok(r);
    }

    print!("{}", uuld);

    let result = client
        .delete_item()
        .table_name("user")
        .key("uid".to_string(), AttributeValue::S(uuld.to_string()))
        .send()
        .await;

    match result {
        Ok(_) => {
            let msg = get_message(vec!["00020".to_string()]).await?;
            let r = format_response(&json!({}), StatusCode::NotFound, &msg).await?;
            return Ok(r);
        }
        Err(z) => {
            print!("{:?}", z);
            let msg = get_message(vec!["00021".to_string()]).await?;
            return Err(LambdaGeneralError { messages: msg });
        }
    }
}

pub async fn get_all() -> Result<Value, LambdaGeneralError<Message>> {
    let client = get_dynamo().await;

    let resp = client.scan().table_name("user").send().await;

    match resp {
        Ok(r) => {
            let result: Vec<User> = r
                .items
                .unwrap()
                .into_iter()
                .map(|f| {
                    User::new(
                        Some(attr_val_to_str(f.get("uid").unwrap()).to_string()),
                        Some(attr_val_to_str(f.get("user_name").unwrap()).to_string()),
                        Some(attr_val_to_str(f.get("password").unwrap()).to_string()),
                        Some(attr_val_to_str(f.get("email").unwrap()).to_string()),
                        Some(attr_val_to_str(f.get("validation_code").unwrap()).to_string()),
                        Some(attr_val_to_str(f.get("ultimo_login").unwrap()).to_string()),
                        Some(attr_val_to_str(f.get("data_cadastro").unwrap()).to_string()),
                        Some(attr_val_to_bool(f.get("habilitado").unwrap())),
                        Some(attr_val_to_str(f.get("first_name").unwrap()).to_string()),
                        Some(attr_val_to_str(f.get("last_name").unwrap()).to_string()),
                        None,
                        Some(attr_val_to_vec(f.get("perfil").unwrap()).to_vec()),
                    )
                    .unwrap()
                })
                .collect();

            if result.len() > 0 {
                let result = json!(result);
                let msg = get_message(vec!["00033".to_string()]).await?;
                let response = format_response(&result, StatusCode::Ok, &msg).await?;
                return Ok(response);
            } else {
                let msg = get_message(vec!["00033".to_string()]).await?;
                let response = format_response(&json!({}), StatusCode::Ok, &msg).await?;
                return Ok(response);
            }
        }
        Err(z) => {
            print!("{:?}", z);
            let msg = get_message(vec!["00049".to_string()]).await?;
            return Err(LambdaGeneralError { messages: msg });
        }
    }
}

pub async fn get_from_credential_enabled(
    user: &UserCredential,
) -> Result<Vec<User>, LambdaGeneralError<Message>> {
    let client = get_dynamo().await;

    let usr = user;

    let crpt = encrypt(usr.password.to_string()).unwrap();

    let crit_express;
    let login_param;
    let login_val;

    if usr.email.trim().is_empty() {
        crit_express = "user_name = :user_name and password = :password and habilitado=:habilitado";
        login_param = ":user_name";
        login_val = usr.user_name.as_str();
    } else if usr.user_name.trim().is_empty() {
        crit_express = "email = :email and password = :password and habilitado=:habilitado";
        login_param = ":email";
        login_val = usr.email.as_str();
    } else {
        crit_express = "email = :email and password = :password and habilitado=:habilitado";
        login_param = ":email";
        login_val = usr.email.as_str();
    }

    let resp = client
        .scan()
        .filter_expression(crit_express)
        .expression_attribute_values(login_param, AttributeValue::S(login_val.to_string()))
        .expression_attribute_values(":password", AttributeValue::S(crpt))
        .expression_attribute_values(":habilitado", AttributeValue::Bool(true))
        .table_name("user")
        .send()
        .await;

    match resp {
        Ok(r) => {
            let result: Vec<User> = r
                .items
                .unwrap()
                .into_iter()
                .map(|f| {
                    User::new(
                        Some(attr_val_to_str(f.get("uid").unwrap()).to_string()),
                        Some(attr_val_to_str(f.get("user_name").unwrap()).to_string()),
                        Some(attr_val_to_str(f.get("password").unwrap()).to_string()),
                        Some(attr_val_to_str(f.get("email").unwrap()).to_string()),
                        Some(attr_val_to_str(f.get("validation_code").unwrap()).to_string()),
                        Some(attr_val_to_str(f.get("ultimo_login").unwrap()).to_string()),
                        Some(attr_val_to_str(f.get("data_cadastro").unwrap()).to_string()),
                        Some(attr_val_to_bool(f.get("habilitado").unwrap())),
                        Some(attr_val_to_str(f.get("first_name").unwrap()).to_string()),
                        Some(attr_val_to_str(f.get("last_name").unwrap()).to_string()),
                        None,
                        Some(attr_val_to_vec(f.get("perfil").unwrap())),
                    )
                    .unwrap()
                })
                .collect();

            Ok(result)
        }
        Err(z) => {
            print!("{:?}", z);
            let msg = get_message(vec!["00050".to_string()]).await?;
            return Err(LambdaGeneralError { messages: msg });
        }
    }
}

pub async fn get_from_email(
    usr: &UserCredential,
) -> Result<Vec<User>, LambdaGeneralError<Message>> {
    let client = get_dynamo().await;

    let resp = client
        .scan()
        .filter_expression("email = :email")
        .expression_attribute_values(":email", AttributeValue::S(usr.email.to_string()))
        .table_name("user")
        .send()
        .await;

    match resp {
        Ok(r) => {
            let result: Vec<User> = r
                .items
                .unwrap()
                .into_iter()
                .map(|f| {
                    User::new(
                        Some(attr_val_to_str(f.get("uid").unwrap()).to_string()),
                        Some(attr_val_to_str(f.get("user_name").unwrap()).to_string()),
                        Some(attr_val_to_str(f.get("password").unwrap()).to_string()),
                        Some(attr_val_to_str(f.get("email").unwrap()).to_string()),
                        Some(attr_val_to_str(f.get("validation_code").unwrap()).to_string()),
                        Some(attr_val_to_str(f.get("ultimo_login").unwrap()).to_string()),
                        Some(attr_val_to_str(f.get("data_cadastro").unwrap()).to_string()),
                        Some(attr_val_to_bool(f.get("habilitado").unwrap())),
                        Some(attr_val_to_str(f.get("first_name").unwrap()).to_string()),
                        Some(attr_val_to_str(f.get("last_name").unwrap()).to_string()),
                        None,
                        Some(attr_val_to_vec(f.get("perfil").unwrap())),
                    )
                    .unwrap()
                })
                .collect();

            Ok(result)
        }
        Err(z) => {
            print!("{:?}", z);
            let msg = get_message(vec!["00051".to_string()]).await?;
            return Err(LambdaGeneralError { messages: msg });
        }
    }
}


pub async fn get_from_id(
    usr: &UserId,
) -> Result<Vec<User>, LambdaGeneralError<Message>> {
    let client = get_dynamo().await;

    let resp = client
        .scan()
        .filter_expression("uid = :uid")
        .expression_attribute_values(":uid", AttributeValue::S(usr.id.to_string()))
        .table_name("user")
        .send()
        .await;

    match resp {
        Ok(r) => {
            let result: Vec<User> = r
                .items
                .unwrap()
                .into_iter()
                .map(|f| {
                    User::new(
                        Some(attr_val_to_str(f.get("uid").unwrap()).to_string()),
                        Some(attr_val_to_str(f.get("user_name").unwrap()).to_string()),
                        Some(attr_val_to_str(f.get("password").unwrap()).to_string()),
                        Some(attr_val_to_str(f.get("email").unwrap()).to_string()),
                        Some(attr_val_to_str(f.get("validation_code").unwrap()).to_string()),
                        Some(attr_val_to_str(f.get("ultimo_login").unwrap()).to_string()),
                        Some(attr_val_to_str(f.get("data_cadastro").unwrap()).to_string()),
                        Some(attr_val_to_bool(f.get("habilitado").unwrap())),
                        Some(attr_val_to_str(f.get("first_name").unwrap()).to_string()),
                        Some(attr_val_to_str(f.get("last_name").unwrap()).to_string()),
                        None,
                        Some(attr_val_to_vec(f.get("perfil").unwrap())),
                    )
                    .unwrap()
                })
                .collect();

            Ok(result)
        }
        Err(z) => {
            print!("{:?}", z);
            let msg = get_message(vec!["00051".to_string()]).await?;
            return Err(LambdaGeneralError { messages: msg });
        }
    }
}


pub async fn update_login_date(user: &UserId) -> Result<bool, LambdaGeneralError<Message>> {
    let client = get_dynamo().await;
    let now = Utc::now();
    let data_login_now: String = format!("{}", now);

    let request = client
        .update_item()
        .table_name("user")
        .key("uid", AttributeValue::S(user.id.to_string()))
        .update_expression("set ultimo_login = :ultimo_login")
        .expression_attribute_values(":ultimo_login", AttributeValue::S(data_login_now));

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

pub async fn assign_game(
    user: &UserId,
    game: &GameId,
) -> Result<Value, LambdaGeneralError<Message>> {
    let uuid = &Uuid::new_v4().to_string();

    let client = get_dynamo().await;

    if is_game_assigned_to_user(user, game).await? {
        let msg = get_message(vec!["00046".to_string()]).await?;
        let resp = format_response(&json!({}), StatusCode::NotAcceptable, &msg).await?;

        return Ok(resp);
    }

    let request = client
        .put_item()
        .table_name("user_game")
        .item("uid", AttributeValue::S(String::from(uuid)))
        .item("user_id", AttributeValue::S(user.id.to_string()))
        .item(
            "game_id",
            AttributeValue::S(String::from(game.id.to_string())),
        );

    let result = request.send().await;

    match result {
        Ok(_) => {
            let msg = get_message(vec!["00041".to_string()]).await?;
            let resp = format_response(&json!({}), StatusCode::Ok, &msg).await?;
            return Ok(resp);
        }
        Err(z) => {
            print!("{:?}", z);
            let msg = get_message(vec!["00042".to_string()]).await?;
            let resp = format_response(&json!({}), StatusCode::Forbidden, &msg).await?;
            return Ok(resp);
        }
    }
}

pub async fn update_validation_code(
    user_id: &UserId,
    user: &UserValidation,
) -> Result<bool, LambdaGeneralError<Message>> {
    let client = get_dynamo().await;

    let vc = &user.validation_code;

    let request = client
        .update_item()
        .table_name("user")
        .key("uid", AttributeValue::S(user_id.id.to_string()))
        .update_expression("set validation_code = :validation_code")
        .expression_attribute_values(":validation_code", AttributeValue::S(vc.to_string()));

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

pub async fn update_password(
    user_id: &UserId,
    user: &UserCredential,
) -> Result<bool, LambdaGeneralError<Message>> {
    let client = get_dynamo().await;

    let request = client
        .update_item()
        .table_name("user")
        .key("uid", AttributeValue::S(user_id.id.to_string()))
        .update_expression("set password = :password")
        .expression_attribute_values(
            ":password",
            AttributeValue::S(encrypt(user.password.to_string()).unwrap()),
        );

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

pub async fn get_validation_code(
    user: &UserCredential,
) -> Result<Value, LambdaGeneralError<Message>> {
    let result = get_from_credential_enabled(&user).await.unwrap();

    if result.len() > 0 {
        println!("[SAGITTARIUS-A]=[{:?}]", result[0]);

        let u = &result[0];

        let res = UserLoginResult {
            id: u.id.to_string(),
            token: "".to_string(),
            user_name: u.user_name.to_string(),
            validation_code: u.validation_code.to_string(),
        };

        let value_respose = json!(res);
        let msg = get_message(vec!["00027".to_string()]).await?;
        let resp = format_response(&value_respose, StatusCode::Ok, &msg).await?;

        return Ok(resp);
    } else {
        let msg = get_message(vec!["00028".to_string()]).await?;
        let resp = format_response(&json!({}), StatusCode::Forbidden, &msg).await?;

        return Ok(resp);
    }
}

pub async fn validate_user_code(
    usr: &UserValidation,
) -> Result<Value, LambdaGeneralError<Message>> {
    let client = get_dynamo().await;

    let email = &usr.email;

    let cre = UserCredential {
        user_name: "".to_string(),
        password: "".to_string(),
        email: email.to_string(),
        expires:0
    };

    let ret_user = get_from_email(&cre).await?;

    if ret_user.len() > 0 {
        if ret_user[0].validation_code == usr.validation_code {
            let cuser_id = ret_user[0].id.to_string();
            let request = client
                .update_item()
                .table_name("user")
                .key("uid", AttributeValue::S(cuser_id))
                .update_expression("set habilitado = :habilitado")
                .expression_attribute_values(":habilitado", AttributeValue::Bool(true));

            let result = request.send().await;

            match result {
                Ok(_) => {
                    let msg = get_message(vec!["00015".to_string()]).await?;
                    let resp = format_response(&json!({}), StatusCode::Ok, &msg).await?;
                    return Ok(resp);
                }
                Err(z) => {
                    print!("{:?}", z);
                    let msg = get_message(vec!["00016".to_string()]).await?;
                    let resp = format_response(&json!({}), StatusCode::Forbidden, &msg).await?;
                    return Ok(resp);
                }
            }
        } else {
            let msg = get_message(vec!["00016".to_string()]).await?;
            let resp = format_response(&json!({}), StatusCode::Forbidden, &msg).await?;
            return Ok(resp);
        }
    } else {
        let msg = get_message(vec!["00016".to_string()]).await?;
        let resp = format_response(&json!({}), StatusCode::Forbidden, &msg).await?;
        return Ok(resp);
    }
}

pub async fn do_recover_password_f1(
    user: &UserCredential,
) -> Result<Value, LambdaGeneralError<Message>> {
    let user = get_from_email(&user).await?;

    let generated_codigo = &Uuid::new_v4().to_string();

    if user.len() > 0 {
        let u_id = UserId {
            id: user[0].id.to_string(),
        };
        let u_val = UserValidation {
            email: user[0].email.to_string(),
            validation_code: generated_codigo.to_string(),
        };

        update_validation_code(&u_id, &u_val).await?;

        let items = vec![
            DictFieldVal {
                field: String::from("@userName@"),
                value: user[0].user_name.to_string(),
            },
            DictFieldVal {
                field: String::from("@generatedCode@"),
                value: generated_codigo.to_string(),
            },
        ];

        let conteudo = get_message_with_fields("00036".to_string(), &items).await?;
        let subject = get_message(vec!["00023".to_string()]).await?;

        send_email(
            "mp@unrealbrasil.com.br",
            user[0].email.as_str(),
            subject[0].mensagem.as_str(),
            conteudo.as_str(),
        )
        .await?;

        let usr_result = UserLoginResult {
            id: user[0].id.to_string(),
            token: "".to_string(),
            user_name: user[0].user_name.to_string(),
            validation_code: "".to_string(),
        };

        let msg = get_message(vec!["00037".to_string()]).await?;
        let resp = format_response(&json!(usr_result), StatusCode::Forbidden, &msg).await?;

        return Ok(resp);
    } else {
        let msg = get_message(vec!["00025".to_string()]).await?;
        let resp = format_response(&json!({}), StatusCode::Forbidden, &msg).await?;

        return Ok(resp);
    }
}

pub async fn do_recover_password_invalid() -> Result<Value, LambdaGeneralError<Message>> {
    let msg = get_message(vec!["00025".to_string()]).await?;
    let resp = format_response(&json!({}), StatusCode::Forbidden, &msg).await?;

    return Ok(resp);
}
pub async fn do_recover_password_f2(
    usr: &UserCredential,
    u_val: &UserValidation,
) -> Result<Value, LambdaGeneralError<Message>> {
    if usr.email.trim().eq("") {
        let msg = get_message(vec!["00008".to_string()]).await?;
        let r = format_response(&json!({}), StatusCode::NotFound, &msg).await?;
        return Ok(r);
    }
    if usr.user_name.trim().eq("") {
        let msg = get_message(vec!["00019".to_string()]).await?;
        let r = format_response(&json!({}), StatusCode::NotFound, &msg).await?;
        return Ok(r);
    }
    if usr.password.trim().eq("") {
        let msg = get_message(vec!["00009".to_string()]).await?;
        let r = format_response(&json!({}), StatusCode::NotFound, &msg).await?;
        return Ok(r);
    }
    if usr.password.len() < 5 {
        let msg = get_message(vec!["00010".to_string()]).await?;
        let r = format_response(&json!({}), StatusCode::NotAcceptable, &msg).await?;
        return Ok(r);
    }

    let user = get_from_email(&usr).await?;

    if user.len() > 0 {
        let in_user = &user[0];

        if u_val.validation_code == in_user.validation_code {
            let uid = UserId {
                id: in_user.id.to_string(),
            };
            let ucred = UserCredential {
                email: in_user.email.to_string(),
                password: usr.password.to_string(),
                user_name: usr.user_name.to_string(),
                expires:0
            };

            let is_updated = update_password(&uid, &ucred).await?;

            if is_updated {
                let items = vec![DictFieldVal {
                    field: String::from("@userName@"),
                    value: user[0].user_name.to_string(),
                }];

                let conteudo = get_message_with_fields("00038".to_string(), &items).await?;
                let subject = get_message(vec!["00023".to_string()]).await?;
                send_email(
                    "mp@unrealbrasil.com.br",
                    usr.email.as_str(),
                    subject[0].mensagem.as_str(),
                    conteudo.as_str(),
                )
                .await?;

                let msg = get_message(vec!["00039".to_string()]).await?;
                let r = format_response(&json!({}), StatusCode::Ok, &msg).await?;

                return Ok(r);
            } else {
                let msg = get_message(vec!["00040".to_string()]).await?;
                let r = format_response(&json!({}), StatusCode::Ok, &msg).await?;

                return Ok(r);
            }
        } else {
            let msg = get_message(vec!["00040".to_string()]).await?;
            let r = format_response(&json!({}), StatusCode::Ok, &msg).await?;

            return Ok(r);
        }
    } else {
        let msg = get_message(vec!["00040".to_string()]).await?;
        let r = format_response(&json!({}), StatusCode::Ok, &msg).await?;

        return Ok(r);
    }
}

pub async fn do_login(cred: &UserCredential) -> Result<Value, LambdaGeneralError<Message>> {
    if cred.email.trim().eq("") && cred.user_name.trim().eq("") {
        let msg = get_message(vec!["00002".to_string()]).await?;
        let r = format_response(&json!({}), StatusCode::NotFound, &msg).await?;
        return Ok(r);
    }

    if cred.password.trim().eq("") {
        let msg = get_message(vec!["00003".to_string()]).await?;
        let r = format_response(&json!({}), StatusCode::NotFound, &msg).await?;
        return Ok(r);
    }

    let usr = get_from_credential_enabled(&cred).await?;

    if usr.len() > 0 {
        let token_str = generate_user_token(&usr[0], cred.expires>=0, cred.expires)
            .await
            .unwrap_or("".to_string());

        let tmp_user = &usr[0];

        let user_id = UserId {
            id: tmp_user.id.to_string(),
        };

        update_login_date(&user_id).await.unwrap();

        let result = UserLoginResult {
            id: tmp_user.id.to_string(),
            token: token_str,
            user_name: usr[0].user_name.to_string(),
            validation_code: "".to_string(),
        };

        let msg = get_message(vec!["00001".to_string()]).await?;
        let r = format_response(&json!(result), StatusCode::Ok, &msg).await?;
        return Ok(r);
    }

    let msg = get_message(vec!["00004".to_string()]).await?;
    let r = format_response(&json!({}), StatusCode::NotFound, &msg).await?;
    return Ok(r);
}

pub async fn email_existis(usr: &User) -> Result<bool, LambdaGeneralError<Message>> {
    let client = get_dynamo().await;

    let resp = client
        .scan()
        .filter_expression("email = :email")
        .expression_attribute_values(":email", AttributeValue::S(usr.email.to_string()))
        .table_name("user")
        .send()
        .await;

    match resp {
        Ok(r) => {
            let itms = r.items.unwrap().len();
            Ok(itms > 0)
        }
        Err(z) => {
            print!("{:?}", z);
            let msg = get_message(vec!["00055".to_string()]).await?;
            return Err(LambdaGeneralError { messages: msg });
        }
    }
}

pub async fn login_existis(usr: &User) -> Result<bool, LambdaGeneralError<Message>> {
    let client = get_dynamo().await;

    let resp = client
        .scan()
        .filter_expression("user_name = :user_name")
        .expression_attribute_values(":user_name", AttributeValue::S(usr.user_name.to_string()))
        .table_name("user")
        .send()
        .await;

    match resp {
        Ok(r) => {
            let itms = r.items.unwrap().len();
            return Ok(itms > 0);
        }
        Err(z) => {
            print!("{:?}", z);
            let msg = get_message(vec!["00054".to_string()]).await?;
            return Err(LambdaGeneralError { messages: msg });
        }
    }
}

pub async fn id_existis(user: &UserId) -> Result<bool, LambdaGeneralError<Message>> {
    let client = get_dynamo().await;

    let resp = client
        .scan()
        .filter_expression("uid = :uid")
        .expression_attribute_values(":uid", AttributeValue::S(user.id.to_string()))
        .table_name("user")
        .send()
        .await;

    match resp {
        Ok(r) => {
            let itms = r.items.unwrap().len();
            return Ok(itms > 0);
        }
        Err(z) => {
            print!("{:?}", z);
            let msg = get_message(vec!["00053".to_string()]).await?;
            return Err(LambdaGeneralError { messages: msg });
        }
    }
}

pub async fn is_game_assigned_to_user(
    user: &UserId,
    game: &GameId,
) -> Result<bool, LambdaGeneralError<Message>> {
    let client = get_dynamo().await;

    let resp = client
        .scan()
        .filter_expression("game_id = :game_id and user_id = :user_id")
        .expression_attribute_values(":user_id", AttributeValue::S(user.id.to_string()))
        .expression_attribute_values(":game_id", AttributeValue::S(game.id.to_string()))
        .table_name("user_game")
        .send()
        .await;

    match resp {
        Ok(r) => {
            let itms = r.items.unwrap().len();
            return Ok(itms > 0);
        }
        Err(z) => {
            print!("{:?}", z);
            let msg = get_message(vec!["00052".to_string()]).await?;
            return Err(LambdaGeneralError { messages: msg });
        }
    }
}
