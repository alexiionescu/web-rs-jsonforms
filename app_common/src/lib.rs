use actix_web::{web, post, HttpRequest};
use app_error::AppError;
use objects::users::UserState;

pub mod schema;
pub mod app_error;
pub mod app_state;

pub mod objects;
mod api_v1;

#[post("/api/v1")]
async fn rest_api(
    app_state: web::Data<app_state::Data>,
    info: web::Json<objects::InfoRequest>,
) -> Result<objects::InfoResponse, AppError> {
    api_v1::request_handler(app_state, info.0).await
}

pub async fn rest_api_get_user_state(app_state: &web::Data<app_state::Data>, http_req: HttpRequest) -> Result<UserState, AppError> {
    api_v1::request_user(app_state, http_req).await
}



