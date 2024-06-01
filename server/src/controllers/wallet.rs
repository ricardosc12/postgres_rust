use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{post, Router},
    Json,
};

use crate::{
    models::wallet::{CreateWalletDto, Wallet},
    utils::{errors::internal_error, response::Response},
    PosgressPool,
};

async fn create_wallet(
    State(pool): State<PosgressPool>,
    Json(wallet): Json<CreateWalletDto>,
) -> Response {

    let res = Wallet::create_wallet(pool, wallet).await?;
    Ok(res)
}

async fn get_wallets(State(pool): State<PosgressPool>) -> Response {
    
    let wallets: Vec<Wallet> = sqlx::query_as("SELECT * FROM public.wallets")
        .fetch_all(&pool)
        .await
        .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(wallets)).into_response())
}

pub fn route_wallets(db: PosgressPool) -> Router {
    Router::new()
        .route("/wallets", post(create_wallet).get(get_wallets))
        .with_state(db)
}
