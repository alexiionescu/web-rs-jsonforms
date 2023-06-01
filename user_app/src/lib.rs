use actix_web::{web, post, HttpRequest};
use app_common::{app_error::AppError, app_state, rest_api_get_user_state};

pub mod objects;
mod api_v1;

#[post("/api/v1/user_app")]
async fn rest_api(
    app_state: web::Data<app_state::Data>,
    info: web::Json<objects::InfoRequest>,
    http_req: HttpRequest
) -> Result<objects::InfoResponse, AppError> {
    let user_state = rest_api_get_user_state(&app_state, http_req).await?;
    api_v1::request_handler(app_state, info.0, user_state).await
}