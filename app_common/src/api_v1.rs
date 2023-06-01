use crate::objects::{
    json_forms,
    users::{self, UserState},
    InfoRequest, InfoResponse,
};
use crate::{app_error::AppError, app_state};
use actix_web::{http::header::AUTHORIZATION, web};

pub async fn request_handler(
    app_state: web::Data<app_state::Data>,
    req: InfoRequest,
) -> Result<InfoResponse, AppError> {
    match req {
        InfoRequest::JsonForms(r) => json_forms::get(r),
        InfoRequest::UsersLogin(r) => {
            let login = {
                let users = app_state.user_list.read().unwrap();
                users.login(&r)
            };
            if let Err(AppError::CacheError) = login {
                web::block(move || users::db_login_user(&app_state, &r)).await?
            } else {
                login
            }
        }
        InfoRequest::UsersNew(r) => web::block(move || users::db_new_user(&app_state, r)).await?,
    }
}

pub(crate) async fn request_user(
    app_state: &web::Data<app_state::Data>,
    http_req: actix_web::HttpRequest,
) -> Result<UserState, AppError> {
    if http_req.headers().contains_key(AUTHORIZATION) {
        if let Ok(token) = http_req.headers().get(AUTHORIZATION).unwrap().to_str() {
            if &token[..6] == "Bearer" {
                let users = app_state.user_list.read().unwrap();
                return users.check_token(&token[7..], app_state.token_expires);
            }
        }
    }
    Err(AppError::InvalidToken)
}
