use std::any;

use actix_web::{http::StatusCode, HttpResponse};
use serde::Serialize;

pub trait Data {}
/// 返回通用 json 数据
#[derive(Serialize)]
pub struct ResponseEntity<T> {
    pub status: Status,
    pub message: String,
    pub data: Option<T>,
}
#[derive(Serialize)]
pub enum Status {
    SUCCESS,
    FAIL,
}

// 返回 json
pub fn json<T: Serialize>(res_body: T) -> HttpResponse {
    HttpResponse::build(StatusCode::OK).json(res_body)
}

// 返回失败 json
pub fn success_response_json<T: Serialize>(message: &str, data: T) -> HttpResponse {
    let res_body: ResponseEntity<T> = ResponseEntity {
        status: Status::SUCCESS,
        message: message.to_string(),
        data: Some(data),
    };

    json(res_body)
}

// 返回成功 json
pub fn error_response_json(message: &str) -> HttpResponse {
    let res_body: ResponseEntity<String> = ResponseEntity {
        status: Status::FAIL,
        message: message.to_string(),
        data: Some("".to_string()),
    };

    json(res_body)
}
