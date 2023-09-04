use aws_sdk_dynamodb::types::AttributeValue;
use serde_json::json;
use serde_json::Value;

use sagittarius_a_utils::helpers::aws_helper::attr_val_to_str;
use sagittarius_a_utils::helpers::aws_helper::get_dynamo;
use sagittarius_a_utils::helpers::message_helper::get_message;
use sagittarius_a_utils::helpers::response_helper::StatusCode;
use sagittarius_a_utils::helpers::response_helper::format_response;
use sagittarius_a_model::splinefigure::Splinefigure;
use sagittarius_a_model::splinefigure::SplinefigurePoint;
use sagittarius_a_model::splinefigure::SplinefigureId;
use sagittarius_a_utils::helpers::{error_helper::LambdaGeneralError, message_helper::Message};

pub async fn get_spline_figure_by_id(spline: &SplinefigureId) -> Result<Value, LambdaGeneralError<Message>> {
    let client = get_dynamo().await;
    
    let resp = client
        .scan()
        .filter_expression("uid = :uid")
        .expression_attribute_values(":uid", AttributeValue::S(spline.id.to_string()))
        .table_name("spline_figure")
        .send()
        .await;

    match resp {
        Ok(r) => {

            let result: Vec<Splinefigure> = r
            .items
            .unwrap()
            .into_iter()
            .map(|f| {
                Splinefigure{
                    id: attr_val_to_str(f.get("uid").unwrap()).to_string(),
                    name: attr_val_to_str(f.get("name").unwrap()).to_string(),
                    points: f.get("points").unwrap().as_m().into_iter().map(|p| {
                        SplinefigurePoint{
                            vertex: attr_val_to_str(p.get("Vertice").unwrap()).to_string(),
                            x_in_tangent: attr_val_to_str(p.get("XInTangent").unwrap()).to_string(),
                            y_in_tangent: attr_val_to_str(p.get("YInTangent").unwrap()).to_string(),
                            z_in_tangent: attr_val_to_str(p.get("ZInTangent").unwrap()).to_string(),
                            x_out_tangent: attr_val_to_str(p.get("XOutTangent").unwrap()).to_string(),
                            y_out_tangent: attr_val_to_str(p.get("YOutTangent").unwrap()).to_string(),
                            z_out_tangent: attr_val_to_str(p.get("ZOutTangent").unwrap()).to_string(),
                            x_pos: attr_val_to_str(p.get("XPos").unwrap()).to_string(),
                            y_pos: attr_val_to_str(p.get("YPos").unwrap()).to_string(),
                            z_pos: attr_val_to_str(p.get("ZPos").unwrap()).to_string()
                        }
                    }).collect(),
                }
            })
            .collect();

            if result.len() > 0 {
                let result = json!(result);
                // TODO: Mudar mensagem
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
            let msg = get_message(vec!["00055".to_string()]).await?;
            return Err(LambdaGeneralError { messages: msg });
        }
    }
}