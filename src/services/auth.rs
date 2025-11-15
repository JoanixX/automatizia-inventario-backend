use crate::models::user::User;
use crate::security::{hashing::verify_password, token::create_token};
use crate::services::users::get_user_by_email;
use anyhow::Result;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login(pool: &PgPool, req: LoginRequest) -> Result<String> {
    let user = get_user_by_email(pool, &req.email)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Invalid credentials"))?;

    let is_valid_password = verify_password(&req.password, &user.password_hash)?;

    if !is_valid_password {
        return Err(anyhow::anyhow!("Invalid credentials"));
    }

    let token = create_token(user.id)?;

    Ok(token)
}
