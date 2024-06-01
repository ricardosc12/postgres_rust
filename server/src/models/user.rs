use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::Type, std::cmp::PartialEq, PartialOrd)]
#[sqlx(type_name = "user_type", rename_all = "lowercase")]
pub enum UserType {
    #[serde(rename = "logista")]
    Logista,
    #[serde(rename = "comum")]
    Comum,
}

#[derive(Debug, Serialize, Deserialize, std::cmp::PartialEq, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub nome: String,
    pub cpf: Option<String>,
    pub cnpj: Option<String>,
    pub email: Option<String>,
    pub senha: Option<String>,
    pub user_type: UserType,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, sqlxinsert::PgInsert)]
pub struct CreateUserDto {
    nome: String,
    cpf: String,
    cnpj: Option<String>,
    email: String,
    senha: String,
    user_type: UserType,
}
