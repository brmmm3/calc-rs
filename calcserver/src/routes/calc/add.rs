use actix_web::{post, web, Responder};
use serde::{Deserialize, Serialize};

use crate::{common::AppState, error::UserError};

#[derive(Deserialize)]
struct Args {
    value: f64,
}

#[derive(Serialize)]
struct Response {
    result: f64,
}

#[post("/add")]
pub async fn post_add(
    app: web::Data<AppState>,
    args: web::Json<Args>,
) -> Result<impl Responder, UserError> {
    if let Ok(mut calc) = app.into_inner().calc.lock() {
        let result = calc.add(args.value);
        return Ok(web::Json(Response { result }));
    }
    Err(UserError::InternalError)
}
