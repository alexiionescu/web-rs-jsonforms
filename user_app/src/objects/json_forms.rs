use super::{InfoResponse, *};
use app_common::{app_error::AppError, JsonFormGUI};
use jsonforms::json_forms::{JsonFormsSerializable};


pub(crate) fn get(r: JsonFormsRequest) -> Result<InfoResponse, AppError> {
    JsonFormGUI!(r.name.as_str(), app::MainRequest, "Main User Form");
    JsonFormGUI!(r.name.as_str(), app::DummyRequest, "Dummy User Form");

    let str_err = format!("user_app unknown requested json schema {}", r.name);
    log::error!("{str_err}");
    Err(AppError::InternalError { msg: str_err })
}