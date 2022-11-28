//#[macro_use]

use sagittarius_a_utils::helpers::message_helper::set_lang;
use lambda_runtime::{service_fn, Error as LambdaError, LambdaEvent};

use serde_json::{json, Value};



use sagittarius_a_controller::{
    itemcompracontroller::{ctrl_get_all_items_from_sku, ctrl_register_new_item, ctrl_notify_users_item_compra_sku},
    matchcontroller::ctrl_register_match_result,
    usercontroller::{
        ctrl_add_new_user, ctrl_assign_game_to_user, ctrl_do_login_user, ctrl_get_all,
        ctrl_get_user_validation_code, ctrl_recover_user_password, ctrl_remove_user,
        ctrl_validate_register_code,
    }, splinefigurecontroller::ctrl_get_spline_figure_from_id, playerachievementcontroller::{ctrl_register_new_player_achievement, ctrl_verify_achievement},
};

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    simple_logger::init_with_level(log::Level::Debug)?;

    let func = service_fn(handler);
    lambda_runtime::run(func).await?;

    Ok(())
}

async fn handler(payload: LambdaEvent<Value>) -> Result<Value, LambdaError> {
    let (event, _context) = payload.into_parts();
    let operation = event["op"].as_str().unwrap_or("invalid");

    set_lang(event["lang"].as_str().unwrap_or("en-us").to_string());

    match operation {
        "getAll" => {
            println!("[SAGITTARIUS-A]=[{}]", operation);

            let resp = ctrl_get_all(&event).await.map_err(|err| {
                println!(
                    "[SAGITTARIUS-A]=[OPERATION: {} , ERROR: {:?}]",
                    operation, err
                );
            });

            Ok(resp.unwrap())
        }
        "doRegisterNewUser" => {
            println!("[SAGITTARIUS-A]=[{}]", operation);
            //let resp = ctrl_add_new_user(event).await;
            let resp = ctrl_add_new_user(&event).await.map_err(|err| {
                println!(
                    "[SAGITTARIUS-A]=[OPERATION: {} , ERROR: {:?}]",
                    operation, err
                );
            });

            Ok(resp.unwrap())
        }
        "doDeleteUserInfos" => {
            println!("[SAGITTARIUS-A]=[{}]", operation);
            //let resp = ctrl_remove_user(event).await;
            let resp = ctrl_remove_user(&event).await.map_err(|err| {
                println!(
                    "[SAGITTARIUS-A]=[OPERATION: {} , ERROR: {:?}]",
                    operation, err
                );
            });

            Ok(resp.unwrap())
        }
        "doLogin" => {
            println!("[SAGITTARIUS-A]=[{}]", operation);
            //let resp = ctrl_do_login_user(event).await;
            let resp = ctrl_do_login_user(&event).await.map_err(|err| {
                println!(
                    "[SAGITTARIUS-A]=[OPERATION: {} , ERROR: {:?}]",
                    operation, err
                );
            });

            Ok(resp.unwrap())
        }
        "doRecoverPassword" => {
            println!("[SAGITTARIUS-A]=[{}]", operation);
            //let resp = ctrl_recover_user_password(event).await;
            let resp = ctrl_recover_user_password(&event).await.map_err(|err| {
                println!(
                    "[SAGITTARIUS-A]=[OPERATION: {} , ERROR: {:?}]",
                    operation, err
                );
            });

            Ok(resp.unwrap())
        }
        "doValidateRegisterCode" => {
            println!("[SAGITTARIUS-A]=[{}]", operation);
            //let resp = ctrl_validate_register_code(event).await;
            let resp = ctrl_validate_register_code(&event).await.map_err(|err| {
                println!(
                    "[SAGITTARIUS-A]=[OPERATION: {} , ERROR: {:?}]",
                    operation, err
                );
            });

            Ok(resp.unwrap())
        }
        "doAssignGameToUser" => {
            println!("[SAGITTARIUS-A]=[{}]", operation);
            //let resp = ctrl_assign_game_to_user(event).await;
            let resp = ctrl_assign_game_to_user(&event).await.map_err(|err| {
                println!(
                    "[SAGITTARIUS-A]=[OPERATION: {} , ERROR: {:?}]",
                    operation, err
                );
            });

            Ok(resp.unwrap())
        }
        "getUserValidationCode" => {
            println!("[SAGITTARIUS-A]=[{}]", operation);
            //let resp = ctrl_get_user_validation_code(event).await;
            let resp = ctrl_get_user_validation_code(&event).await.map_err(|err| {
                println!(
                    "[SAGITTARIUS-A]=[OPERATION: {} , ERROR: {:?}]",
                    operation, err
                );
            });

            Ok(resp.unwrap())
        }
        "doRegisterMatchResult" => {
            println!("[SAGITTARIUS-A]=[{}]", operation);
            //let resp = ctrl_register_match_result(event).await;
            let resp = ctrl_register_match_result(&event).await.map_err(|err| {
                println!(
                    "[SAGITTARIUS-A]=[OPERATION: {} , ERROR: {:?}]",
                    operation, err
                );
            });

            Ok(resp.unwrap())
        }
        "doRegisterNewPurchaseItem" => {
            println!("[SAGITTARIUS-A]=[{}]", operation);
            let resp = ctrl_register_new_item(&event).await.map_err(|err| {
                println!(
                    "[SAGITTARIUS-A]=[OPERATION: {} , ERROR: {:?}]",
                    operation, err
                );
            });

            Ok(resp.unwrap())
        }

        "getAllPurchaseItemFromSku" => {
            println!("[SAGITTARIUS-A]=[{}]", operation);
            let resp = ctrl_get_all_items_from_sku(&event).await.map_err(|err| {
                println!(
                    "[SAGITTARIUS-A]=[OPERATION: {} , ERROR: {:?}]",
                    operation, err
                );
            });

            Ok(resp.unwrap())
        }
        "getSplineFigureFromId" => {
            println!("[SAGITTARIUS-A]=[{}]", operation);
            
            let resp = ctrl_get_spline_figure_from_id(&event).await.map_err(|err| {
                println!(
                    "[SAGITTARIUS-A]=[OPERATION: {} , ERROR: {:?}]",
                    operation, err
                );
            });

            Ok(resp.unwrap())
        }
        "doRegisterNewPlayerAchievement" => {
            println!("[SAGITTARIUS-A]=[{}]", operation);
            
            let resp = ctrl_register_new_player_achievement(&event).await.map_err(|err| {
                println!(
                    "[SAGITTARIUS-A]=[OPERATION: {} , ERROR: {:?}]",
                    operation, err
                );
            });

            Ok(resp.unwrap())
        }
        "doVerifyAchievement" => {
            println!("[SAGITTARIUS-A]=[{}]", operation);
            
            let resp = ctrl_verify_achievement(&event).await.map_err(|err| {
                println!(
                    "[SAGITTARIUS-A]=[OPERATION: {} , ERROR: {:?}]",
                    operation, err
                );
            });

            Ok(resp.unwrap())
        }
        "doNotifyUsersItemCompra" => {
            println!("[SAGITTARIUS-A]=[{}]", operation);
            
            let resp = ctrl_notify_users_item_compra_sku(&event).await.map_err(|err| {
                println!(
                    "[SAGITTARIUS-A]=[OPERATION: {} , ERROR: {:?}]",
                    operation, err
                );
            });

            Ok(resp.unwrap())
        }        


        _ => {
            println!("[SAGITTARIUS-A]=[!!INVALIDO!!]");

            let invalid_result = json!({
                "statusCode": "0",
                "body": "{INVALID}",
                "isBase64Encoded": false
            });

            return Ok(invalid_result);
        }
    }
}
