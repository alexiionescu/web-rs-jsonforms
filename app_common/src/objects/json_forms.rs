use super::{InfoResponse, *};
use crate::{app_error::AppError, JsonFormGUI};
use jsonforms::json_forms::{JsonFormsSerializable, *};

#[macro_export]
macro_rules! JsonFormGUI {
    ($e:expr, $t:ty,$s:expr) => {
        if $e == stringify!($t) {
            let mut form = JsonFormsResponse::from(<$t>::jsonforms_schema());
            <$t>::add_buttons(&mut form);
            form.title = Some($s.to_owned());
            return Ok(InfoResponse {
                response: Some(ApiResponse::JsonForms(form)),
                user_state: None,
            });
        }
    };
}

pub(crate) fn get(r: JsonFormsRequest) -> Result<InfoResponse, AppError> {
    JsonFormGUI!(r.name.as_str(), users::LoginRequest, "Login");
    JsonFormGUI!(r.name.as_str(), users::NewRequest, "User Sign Up");

    let str_err = format!("unknown requested json schema {}", r.name);
    log::error!("{str_err}");
    Err(AppError::InternalError { msg: str_err })
}
