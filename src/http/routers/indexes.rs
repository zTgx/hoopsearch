use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use crate::http::response::ResponseBody;
use crate::http::error::ResponseError;

pub async fn get_indexes() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(ResponseBody::new(222, "OK", "get_indexes")))
}

pub async fn get_index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(ResponseBody::new(222, "OK", "get_index")))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IndexCreateRequest {
    name        : Option<String>,
    uid         : Option<String>,
    primary_key : Option<String>,
}

// 创建index接口
// 参数: IndexCreateRequest
// pub async fn create_index() -> Result<HttpResponse, ResponseError> {
pub async fn create_index(body: web::Json<IndexCreateRequest>) -> Result<HttpResponse, ResponseError> {
    info!("创建 index 参数: {:?}", body);

    // uid 和 name 不能都为空
    if let (None, None) = (body.name.clone(), body.uid.clone()) {
        return Err(ResponseError::bad_request("Index的uid参数不能为空"));
    }



    Ok(HttpResponse::Ok().json(ResponseBody::new(222, "OK", "create_index")))
}

pub async fn update_index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(ResponseBody::new(222, "OK", "update_index")))
}

pub async fn delete_index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(ResponseBody::new(222, "OK", "delete_index")))
}