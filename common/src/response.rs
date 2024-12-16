use axum::Json;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde::Serialize;
use thiserror::Error;


#[derive(Serialize, Debug)]
pub struct ErrorDetails {
    pub code: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct BaseApiResponse<T, E>
where
    T: Serialize,
    E: Serialize,
{
    pub status: String,
    pub message: String,
    pub data: Option<T>,
    pub errors: Option<E>,
}

impl<T, E> BaseApiResponse<T, E>
where
    T: Serialize,
    E: Serialize,
{
    pub fn new(status: &str, message: &str, data: Option<T>, errors: Option<E>) -> Self {
        BaseApiResponse {
            status: status.to_string(),
            message: message.to_string(),
            data,
            errors,
        }
    }

    pub fn with_status_code(self, code: StatusCode) -> Response {
        let mut response = Json(self).into_response(); // Convert to a full response
        *response.status_mut() = code; // Modify status code on the entire response object
        response
    }
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Not Found")]
    NotFound,
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Bad Request")]
    BadRequest,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
}

impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::BadRequest => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden => StatusCode::FORBIDDEN,
        }
    }

    pub fn to_response<T>(self) -> Response
    where
        T: Serialize,
    {
        let error_details = ErrorDetails {
            code: self.to_string(),
            message: self.to_string(),
        };
        let response = BaseApiResponse::<T, ErrorDetails>::new(
            "error",
            &self.to_string(),
            None,
            Some(error_details),
        );
        response.with_status_code(self.status_code())
    }
}