use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response as ResponseAxum},
    routing::{delete, get},
    Json, Router,
};

use crate::{
    models::user::{CreateUserDto, User},
    utils::response::Response,
    PosgressPool,
};

async fn get_users(
    State(pool): State<PosgressPool>,
) -> Result<ResponseAxum<Body>, ResponseAxum<Body>> {
    let users: Vec<User> = sqlx::query_as("SELECT * FROM public.users")
        .fetch_all(&pool)
        .await
        .unwrap();

    Ok((StatusCode::OK, Json(users)).into_response())
}

async fn create_user(
    State(pool): State<PosgressPool>,
    Json(user): Json<CreateUserDto>,
) -> Result<ResponseAxum<Body>, ResponseAxum<Body>> {
    let res = user.insert::<User>(&pool, "public.users").await;

    if res.is_err() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            res.unwrap_err().to_string(),
        )
            .into_response());
    }

    Ok((StatusCode::OK, Json(res.unwrap())).into_response())
}

async fn delete_user(
    State(pool): State<PosgressPool>,
    Path(user_id): Path<i32>,
) -> Result<ResponseAxum<Body>, ResponseAxum<Body>> {
    let res = sqlx::query("DELETE FROM public.users as u WHERE u.id = $1")
        .bind(user_id)
        .execute(&pool)
        .await;

    if res.is_err() {
        return Err((StatusCode::OK, Json(res.unwrap_err().to_string())).into_response());
    }

    if res.unwrap().rows_affected() == 0 {
        return Ok((StatusCode::NOT_FOUND).into_response());
    }

    Ok((StatusCode::OK).into_response())
}

async fn get_user_by_email(
    State(pool): State<PosgressPool>,
    Path(user_email): Path<String>,
) -> Response {
    let user = User::get_user_by_email(&user_email, pool).await?;

    Ok((StatusCode::OK, Json(user)).into_response())
}

pub fn route_users(pool: PosgressPool) -> Router {
    let router: Router = Router::new()
        .route("/users", get(get_users).post(create_user))
        .route(
            "/users/:user_id",
            delete(delete_user).get(get_user_by_email),
        )
        // .route("/users/transfer", post(transfer_to))
        .with_state(pool);

    return router;
}
