use axum::Json;
use serde_json::{Value, json};

use crate::{error, utils};

pub async fn get_printer() -> Result<Json<Value>, error::HttpError> {
    let prints = utils::get_printers()?;
    Ok(Json(json!({
        "code": 200,
        "success": true,
        "data":prints,
    })))
}
