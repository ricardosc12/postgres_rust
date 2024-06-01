use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::Type, std::cmp::PartialEq, PartialOrd)]
#[sqlx(type_name = "currency")]
pub enum Currency {
    BRL,
    USD,
    EUR,
    CNY,
    JPY,
}

#[derive(Debug, Serialize, Deserialize, std::cmp::PartialEq, sqlx::FromRow, sqlxinsert::PgInsert)]
pub struct Wallet {
    pub id: i32,
    pub user_id: i32,
    pub currency: Currency,
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, sqlxinsert::PgInsert)]
pub struct CreateWalletDto {
    pub user_id: i32,
    pub currency: Currency,
}

// #[derive(Debug, ToSql)]
// struct Amount(i32);
