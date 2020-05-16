use actix_web::{HttpResponse, Result};
use crate::http::response::ResponseBody;

pub async fn get_indexes() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(ResponseBody::new(222, "OK", "get_indexes")))
}

pub async fn get_index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(ResponseBody::new(222, "OK", "get_index")))
}