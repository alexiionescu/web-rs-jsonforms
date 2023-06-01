
use actix_web::{Responder, body::BoxBody, HttpRequest, HttpResponse, http::header::ContentType};
use app_common::objects::users::UserState;
use serde::{Deserialize, Serialize};
use jsonforms::json_forms::*;
use user_app_common::objects as CommonObjects;

pub mod app;
pub mod json_forms;

#[derive(Deserialize)]
pub enum InfoRequest {
    JsonForms(JsonFormsRequest),
    AppMain(app::MainRequest),
}

#[derive(Serialize)]
pub struct InfoResponse {
    response: Option<ApiResponse>,
    user_state: Option<UserState>,
}

#[derive(Serialize)]
pub enum ApiResponse {
    JsonForms(JsonFormsResponse),
    AppMain(CommonObjects::app::MainResponse),
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