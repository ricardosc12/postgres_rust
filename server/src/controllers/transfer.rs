use axum::{
    extract::State,
    routing::{post, Router},
    Json,
};

use crate::{
    models::transfer::{CreateTransferDto, Transference},
    utils::response::Response,
    PosgressPool,
};

async fn create_transference(
    State(pool): State<PosgressPool>,
    Json(transference): Json<CreateTransferDto>,
) -> Response {
    Transference::transfer(pool, transference, false).await
}

async fn create_transference_slow(
    State(pool): State<PosgressPool>,
    Json(transference): Json<CreateTransferDto>,
) -> Response {
    Transference::transfer(pool, transference, true).await
}

async fn get_transferences(State(pool): State<PosgressPool>) -> Response {
    Transference::get_transferences(pool).await
}

pub fn route_transferences(db: PosgressPool) -> Router {
    Router::new()
        .route(
            "/transfer",
            post(create_transference).get(get_transferences),
        )
        .route("/transfer_slow", post(create_transference_slow))
        .with_state(db)
}
