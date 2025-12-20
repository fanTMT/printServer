use crate::{
    AppState,
    config::{default_config_path, save_config},
    error::{self, HttpError},
};
use anyhow::anyhow;
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Deserialize, Serialize, Debug)]
pub struct Conf {
    isauto: bool,
    orientation: u64,
    page_size: String,
    printer: String,
}
pub async fn set_setting(
    State(state): State<AppState>,
    Json(payload): Json<Conf>,
) -> Result<Json<Value>, error::HttpError> {
    // println!("payload{:#?}", payload);
    let mut conf = state
        .config
        .write()
        .map_err(|e| HttpError::Internal(anyhow!("读取配置失败（锁污染）: {}", e)))?;

    conf.printer.printer = payload.printer.clone();
    conf.printer.page_size = payload.page_size.clone();
    conf.printer.orientation = payload.orientation;
    conf.printer.enabled_auto_print = payload.isauto;
    let config_path = default_config_path().expect("配置文件路径");
    save_config(&conf, config_path)?;
    Ok(Json(json!({
        "code": 200,
        "success": true,
        "data":payload,
    })))
}

pub async fn get_setting(State(state): State<AppState>) -> Result<Json<Value>, error::HttpError> {
    let config = state
        .config
        .read()
        .map_err(|e| HttpError::Internal(anyhow!("读取配置失败（锁污染）: {}", e)))?;

    let value = json!({
        "code": 200,
        "success": true,
        "data": config.printer.clone()
    });
    Ok(Json(value))
}
