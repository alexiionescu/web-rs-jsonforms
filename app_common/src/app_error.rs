use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};
#[derive(Debug, Display, Error)]
pub enum AppError {
    #[display(fmt = "Invalid Input Data: {field} {msg}")]
    ValidationError {
        field: &'static str,
        msg: &'static str,
    },
    #[display(fmt = "Internal Error {msg}")]
    InternalError {
        msg: String,
    },
    CacheError,
    InvalidUser,
    InvalidToken,
}

impl From<actix_web::error::BlockingError> for AppError {
    fn from(err: actix_web::error::BlockingError) -> Self {
        let block_err = format!("actix web blocking Error: {:?}", err);
        log::error!("{block_err}");
        AppError::InternalError { msg: block_err }
    }
}

impl From<std::fmt::Error> for AppError {
    fn from(err: std::fmt::Error) -> Self {
        let err_msg = format!("format Error {:?}", err);
        log::error!("{err_msg}");
        AppError::InternalError { msg: err_msg }
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(err: diesel::result::Error) -> Self {
        let str_err = format!("Diesel Error {:?}", err);
        log::error!("{str_err}");
        AppError::InternalError { msg: str_err }
    }
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            AppError::InvalidToken | AppError::InvalidUser => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
