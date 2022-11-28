use serde_json::{json, Value};

use sagittarius_a_utils::helpers::{
    error_helper::LambdaGeneralError,
    message_helper::{get_message, Message},
    response_helper::{format_response, StatusCode},
};
use sagittarius_a_model::{
    splinefigure::SplinefigureId
};

use sagittarius_a_service::{
    splinefigureservice::get_spline_figure_by_id,
};


pub async fn ctrl_get_spline_figure_from_id(
    event: &Value,
) -> Result<Value, LambdaGeneralError<Message>> {

    let id = SplinefigureId {
        id: event["id"].as_str().unwrap_or("0").to_string(),
    };

    let list = get_spline_figure_by_id(&id).await?;

    let msg = get_message(vec!["00062".to_string()]).await?;
    let r = format_response(&json!(list), StatusCode::Ok, &msg).await?;

    return Ok(r);
}
