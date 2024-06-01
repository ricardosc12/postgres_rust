use std::time::Instant;

use crate::{
    models::user::User,
    utils::{errors::internal_error, response::ResponseError},
    PosgressPool,
};

impl User {
    pub async fn get_user_by_email(
        email: &String,
        pool: PosgressPool,
    ) -> Result<User, ResponseError> {

        let now = Instant::now();

        let user: User = sqlx::query_as("SELECT * FROM users WHERE cpf=$1")
            .bind(email)
            .fetch_one(&pool)
            .await
            .map_err(internal_error)?;

        let elapsed = now.elapsed();

        println!("Elapsed: {:.2?}", elapsed);        

        Ok(user)
    }
}
