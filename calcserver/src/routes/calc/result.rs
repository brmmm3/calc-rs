use actix_web::{ get, web, Responder };
use serde::Serialize;

use crate::{ common::AppState, error::UserError };

#[derive(Serialize)]
struct Response {
    result: f64,
}

#[get("/result")]
pub async fn get_result(app: web::Data<AppState>) -> Result<impl Responder, UserError> {
    if let Ok(calc) = app.into_inner().calc.lock() {
        return Ok(
            web::Json(Response {
                result: calc.result(),
            })
        );
    }
    Err(UserError::InternalError)
}
