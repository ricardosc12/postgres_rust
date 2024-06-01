use std::{env, str::FromStr};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use postgres_types::{FromSql, ToSql};
use serde::Serialize;
// use sqlx::prelude::FromRow;
use tokio_postgres::{Config, Error, NoTls, Row};
use utils::{errors::internal_error, response::Response};

// use sqlx::{
//     postgres::{PgPoolOptions, Postgres},
//     Pool,
// };

// mod controllers;
// mod models;
// mod services;
mod utils;

// use controllers::transfer::route_transferences;
// use controllers::users::route_users;
// use controllers::wallet::route_wallets;

#[derive(Clone, Debug, ToSql, FromSql, Serialize)]
#[postgres(name = "user_type")]
#[serde(rename_all = "lowercase")]
enum UserType {
    #[postgres(name = "logista")]
    Logista,
    #[postgres(name = "comum")]
    Comum,
}

#[derive(Clone, Debug, Serialize)]
struct User {
    id: i32,
    nome: String,
    cpf: String,
    cnpj: String,
    email: String,
    senha: String,
    user_type: UserType,
}

impl User {
    fn from_one(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            nome: row.get("nome"),
            cpf: row.get("cpf"),
            cnpj: row.get("cnpj"),
            email: row.get("email"),
            senha: row.get("senha"),
            user_type: row.get("user_type"),
        }
    }
}

pub type PosgressPool = Pool<PostgresConnectionManager<NoTls>>;

async fn test(State(pool): State<PosgressPool>, Path(user_id): Path<String>) -> Response {
    let conn = pool.get().await.map_err(internal_error)?;

    let row = conn
        .query_one("SELECT * FROM users WHERE cpf=$1", &[&user_id])
        .await
        .map_err(internal_error)?;

    let user: User = User::from_one(&row);

    Ok((StatusCode::OK, Json(user)).into_response())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let port = env::var("PORT").unwrap_or("5000".to_string());

    let pg_config = Config::from_str("postgresql://postgres:password@localhost/payment_system")?;
    let pg_manager = PostgresConnectionManager::new(pg_config, NoTls);

    let pg_pool: PosgressPool = Pool::builder().max_size(300).build(pg_manager).await?;

    // let db_pool: Pool<Postgres> = PgPoolOptions::new()
    //     .min_connections(1)
    //     .max_connections(3)
    //     .connect("postgresql://postgres:password@localhost/payment_system")
    //     .await
    //     .unwrap();

    let app: Router = Router::new().route("/users/:user_id", get(test).with_state(pg_pool));
    //     .merge(route_users(db_pool.clone()))
    //     .merge(route_wallets(db_pool.clone()))
    //     .merge(route_transferences(db_pool.clone()));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

