use actix_web::{Responder, body::BoxBody, HttpRequest, HttpResponse, http::header::ContentType};
use serde::{Deserialize, Serialize};
pub mod json_forms;
use jsonforms::json_forms::*;

use self::users::UserState;
pub mod users;

#[derive(Deserialize)]
pub enum InfoRequest {
    JsonForms(JsonFormsRequest),
    UsersLogin(users::LoginRequest),
    UsersNew(users::NewRequest),
}

#[derive(Serialize,Debug)]
pub struct InfoResponse {
    response: Option<ApiResponse>,
    user_state: Option<UserState>,
}

#[derive(Serialize,Debug)]
pub enum ApiResponse {
    JsonForms(JsonFormsResponse),
    UsersLogin(users::LoginResponse),
}

impl Responder for InfoResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}


