use axum::{body::Body, http::{Response, StatusCode}, response::IntoResponse};

pub fn internal_error<E>(err: E) -> Response<Body>
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
}
