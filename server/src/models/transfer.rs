use super::wallet::Currency;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::Type, std::cmp::PartialEq, PartialOrd)]
#[sqlx(type_name = "transfer_status", rename_all = "lowercase")]
pub enum TransferStatus {
    PENDING,
    CANCELED,
    CONCLUDED,
    FAILED,
}

#[derive(Debug, Serialize, Deserialize, std::cmp::PartialEq, sqlx::FromRow)]
pub struct Transference {
    pub id: i32,
    pub payer_id: i32,
    pub payee_id: i32,
    pub amount: f64,
    pub wallet_payer_id: i32,
    pub wallet_payee_id: i32,
    pub currency: Currency,
    pub status: TransferStatus,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, sqlxinsert::PgInsert)]
pub struct CreateTransferDto {
    pub payer_id: i32,
    pub payee_id: i32,
    pub amount: f64,
    pub currency: Currency,
}
