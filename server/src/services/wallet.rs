use crate::{
    models::wallet::{CreateWalletDto, Currency, Wallet},
    utils::{
        errors::internal_error,
        response::{Response, ResponseError},
    },
    PosgressPool,
};
use axum::{http::StatusCode, response::IntoResponse, Json};
use sqlx::{Executor, Postgres};

impl Wallet {
    pub async fn create_wallet(pool: PosgressPool, wallet: CreateWalletDto) -> Response {
        let wallet = wallet
            .insert::<Wallet>(&pool, "public.wallets")
            .await
            .map_err(internal_error)?;

        Ok((StatusCode::OK, Json(wallet)).into_response())
    }

    pub async fn get_wallet(
        pool: &PosgressPool,
        user_id: &i32,
        currency: &Currency,
    ) -> Result<Wallet, ResponseError> {
        let wallet: Wallet =
            sqlx::query_as("SELECT * FROM wallets WHERE user_id=$1 AND currency=$2")
                .bind(user_id)
                .bind(currency)
                .fetch_one(pool)
                .await
                .map_err(internal_error)?;

        Ok(wallet)
    }

    pub async fn increment_amount<'a, E>(&self, pool: E, amount: f64) -> Result<(), ResponseError>
    where
        E: Executor<'a, Database = Postgres>,
    {
        let _ = sqlx::query("UPDATE wallets SET amount=amount+$1 WHERE id=$2")
            .bind(amount)
            .bind(self.id)
            .execute(pool)
            .await
            .map_err(internal_error)?;

        Ok(())
    }

    pub async fn decrement_amount<'a, E>(&self, pool: E, amount: f64) -> Result<(), ResponseError>
    where
        E: Executor<'a, Database = Postgres>,
    {
        let _ = sqlx::query("UPDATE wallets SET amount=amount-$1 WHERE id=$2")
            .bind(amount)
            .bind(self.id)
            .execute(pool)
            .await
            .map_err(internal_error)?;

        Ok(())
    }
}

