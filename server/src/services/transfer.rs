use axum::{http::StatusCode, response::IntoResponse, Json};
use sqlx::{Executor, Postgres};
use std::time;

use crate::{
    models::{
        transfer::{CreateTransferDto, TransferStatus, Transference},
        wallet::Wallet,
    },
    utils::{errors::internal_error, response::Response},
    PosgressPool,
};

impl Transference {
    pub async fn get_transferences(pool: PosgressPool) -> Response {
        let transferences: Vec<Transference> = sqlx::query_as(
            "
            SELECT * FROM transfers;
            ",
        )
        .fetch_all(&pool)
        .await
        .map_err(internal_error)?;

        Ok((StatusCode::OK, Json(transferences)).into_response())
    }
    pub async fn transfer(
        pool: PosgressPool,
        transference: CreateTransferDto,
        slow: bool,
    ) -> Response {
        if transference.payee_id == transference.payer_id {
            return Err((
                StatusCode::FORBIDDEN,
                Json("Transferências não podem ocorrer entre mesmos usuários"),
            )
                .into_response());
        }

        let payer_wallet =
            Wallet::get_wallet(&pool, &transference.payer_id, &transference.currency).await?;

        if payer_wallet.amount < transference.amount {
            return Err((StatusCode::FORBIDDEN, Json("Sem fundos suficientes")).into_response());
        }

        let payee_wallet =
            Wallet::get_wallet(&pool, &transference.payee_id, &transference.currency).await?;

        let mut transaction = pool.begin().await.map_err(internal_error)?;

        payer_wallet
            .decrement_amount(&mut *transaction, transference.amount)
            .await?;

        if slow {
            // Create concurrency problem, deadlock
            tokio::time::sleep(time::Duration::from_millis(10000)).await
        }

        payee_wallet
            .increment_amount(&mut *transaction, transference.amount)
            .await?;

        let new_transference = Transference {
            id: 0,
            payer_id: transference.payer_id,
            payee_id: transference.payee_id,
            wallet_payer_id: payer_wallet.id,
            wallet_payee_id: payee_wallet.id,
            currency: transference.currency,
            status: TransferStatus::CONCLUDED,
            amount: transference.amount,
        };

        Transference::insert(&mut *transaction, new_transference).await?;

        let _ = transaction.commit().await.map_err(internal_error)?;

        Ok((StatusCode::OK).into_response())
    }

    pub async fn insert<'a, E>(pool: E, transference: Transference) -> Response
    where
        E: Executor<'a, Database = Postgres>,
    {
        let _ = sqlx::query(
            "INSERT INTO transfers(payer_id, payee_id, amount, wallet_payer_id, wallet_payee_id, currency, status) VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(transference.payer_id)
        .bind(transference.payee_id)
        .bind(transference.amount)
        .bind(transference.wallet_payer_id)
        .bind(transference.wallet_payee_id)
        .bind(transference.currency)
        .bind(transference.status)
        .execute(pool).await.map_err(internal_error)?;

        Ok((StatusCode::OK).into_response())
    }
}
