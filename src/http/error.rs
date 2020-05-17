use crate::http::response::ResponseBody;
use actix_web::{
    error,
    HttpResponse,
    http::StatusCode,
};
use std::fmt;

pub struct ServiceError {
    pub http_status: StatusCode,
    pub body: ResponseBody<String>,
}

impl ServiceError {
    pub fn new(http_status: StatusCode, message: String) -> ServiceError {
        ServiceError {
            http_status,
            body: ResponseBody {
                code: http_status.as_u16() as u32,
                message,
                data: String::new(),
            }
        }
    }

    pub fn response(&self) -> HttpResponse {
        HttpResponse::build(self.http_status).json(&self.body)
    }
}

// 返回数据错误
#[derive(Debug)]
pub enum ResponseError {
    BadRequest(String),
}
impl ResponseError {
    pub fn bad_request(err: impl fmt::Display) -> ResponseError {
        ResponseError::BadRequest(err.to_string())
    }
}
impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadRequest(err) => f.write_str(err),
        }
    }
}
impl error::ResponseError for ResponseError {
    fn error_response(&self) -> HttpResponse {
        ServiceError::new(self.status_code(), self.to_string()).response()
    }

    fn status_code(&self) -> StatusCode {
       match *self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }
}