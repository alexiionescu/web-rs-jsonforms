use actix_web::web;
use app_common::{app_error::AppError, app_state, objects::users::UserState};
use crate::{
    objects::{json_forms, InfoRequest, InfoResponse, app},
};

pub async fn request_handler(
    app_state: web::Data<app_state::Data>,
    req: InfoRequest,
    user_state: UserState
) -> Result<InfoResponse, AppError> {
    match req {
        InfoRequest::JsonForms(r) => json_forms::get(r),
        InfoRequest::AppMain(r) => app::handle_main_request(&app_state, user_state, r)  
    }
}