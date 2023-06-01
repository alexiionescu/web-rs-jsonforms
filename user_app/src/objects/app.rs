use app_common::{app_error::AppError, app_state::Data as AppData, objects::users::UserState};
use jsonforms::json_forms::*;
use jsonforms_derive::JsonForms;
use serde::Deserialize;
use user_app_common::objects::app::MainResponse;

use super::{ApiResponse, InfoResponse};

#[derive(Deserialize, JsonForms)]
pub struct MainRequest {
    pub app_str: String,
}

impl JsonFormsButtons for MainRequest {
    fn add_buttons(form: &mut JsonFormsResponse) {
        form.add_button(Button {
            name: "Submit",
            btype: ButtonType::Submit,
            bpos: ButtonPos::Center,
            form: None,
        });
    }
}

pub fn handle_main_request(
    app_state: &AppData,
    mut user_state: UserState,
    r: MainRequest,
) -> Result<InfoResponse, AppError> {
    user_state.json_form.name = stringify_nosp!(app::DummyRequest);
    let res_state = user_state.clone();
    user_state.save(app_state)?;
    Ok(InfoResponse {
        response: Some(ApiResponse::AppMain(MainResponse {
            hello: format!("hello {}", r.app_str),
        })),
        user_state: Some(res_state),
    })
}

#[derive(Deserialize, JsonForms)]
pub struct DummyRequest {
    pub dummy_str: String,
}

impl JsonFormsButtons for DummyRequest {
    fn add_buttons(form: &mut JsonFormsResponse) {
        form.add_button(Button {
            name: "Submit",
            btype: ButtonType::Submit,
            bpos: ButtonPos::Center,
            form: None,
        });
        form.add_button(Button {
            name: "Go To Main",
            btype: ButtonType::NextForm,
            bpos: ButtonPos::Right,
            form: Some(JsonFormsRequest {
                name: stringify_nosp!(app::MainRequest),
            }),
        });
    }
}