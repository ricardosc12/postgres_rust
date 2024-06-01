use axum::{body::Body, response::Response as ResponseAxum};

pub type Response = Result<ResponseAxum<Body>, ResponseAxum<Body>>;

pub type ResponseError = ResponseAxum<Body>;