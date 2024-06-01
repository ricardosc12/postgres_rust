use axum::Router;

use sqlx::{
    postgres::{PgPoolOptions, Postgres},
    Pool,
};

mod controllers;
mod models;
mod services;
mod utils;

use controllers::transfer::route_transferences;
use controllers::users::route_users;
use controllers::wallet::route_wallets;

pub type PosgressPool = Pool<Postgres>;

#[tokio::main]
async fn main() {
    
    let db_pool: Pool<Postgres> = PgPoolOptions::new()
        .min_connections(1)
        .max_connections(3)
        .connect("postgresql://postgres:password@localhost/payment_system")
        .await
        .unwrap();

    let app: Router = Router::new()
        .merge(route_users(db_pool.clone()))
        .merge(route_wallets(db_pool.clone()))
        .merge(route_transferences(db_pool.clone()));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", 5000))
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

// #[tokio::main]
// async fn main() {
//     //postgresql://[user[:password]@][netloc][:port][/dbname][?param1=value1&...] DATABASE_URL
//     let (_, _) = tokio::join!(main_process(5000), main_process(5001));
// }
