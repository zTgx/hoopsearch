use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::http::response::ResponseBody;
use crate::http::error::ResponseError;
use crate::http::data::Data;

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
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct IndexResponse {
    name        : String,
    uid         : String,
    created_at  : DateTime<Utc>,
    updated_at  : DateTime<Utc>,
    primary_key : Option<String>,
}

// 创建index接口
// 参数: IndexCreateRequest
// pub async fn create_index() -> Result<HttpResponse, ResponseError> {
pub async fn create_index(data: web::Data<Data>, body: web::Json<IndexCreateRequest>) -> Result<HttpResponse, ResponseError> {
    info!("创建 index 参数: {:?}", body);

    // uid 和 name 不能都为空
    if let (None, None) = (body.name.clone(), body.uid.clone()) {
        return Err(ResponseError::bad_request("Index的uid/name参数不能都为空"));
    }

    // uid
    let uid = match &body.uid {
        Some(uid) => {
            if uid
                .chars()
                .all(|x| x.is_ascii_alphanumeric() || x == '-' || x == '_')
            {
                uid.to_owned()
            } else {
                return Err(ResponseError::bad_request("Index的uid参数无效"));
            }
        }
        None =>  {
            return Err(ResponseError::bad_request("Index的uid参数不能为空"));
        }
    };

    // name
    let name = body.name.as_ref().unwrap_or(&uid);

    // created_at
    let created_at = Utc::now();

    // updated_at
    let updated_at = Utc::now();

    // primary_key
    let primary_key = None;

    match data.db.create_index(&uid) {
        Ok(_) => {
            Ok(
                HttpResponse::Ok().json(
                    IndexResponse {
                        name: name.to_string(),
                        uid,
                        created_at,
                        updated_at,
                        primary_key,
                    }
                )
            )        
        }
        Err(err) => Err(ResponseError::bad_request(err))
    }
}

pub async fn update_index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(ResponseBody::new(222, "OK", "update_index")))
}

pub async fn delete_index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(ResponseBody::new(222, "OK", "delete_index")))
}