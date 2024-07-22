use crate::result::res::success_response_json;
use actix_web::{Error, HttpResponse, Result};

pub async fn create_menu() -> Result<HttpResponse, Error> {
    Ok(success_response_json("createMenuOK", ""))
}
