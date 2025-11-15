use crate::models::user::{NewUser, UpdateUser, User};
use crate::security::hashing::hash_password;
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_user(pool: &PgPool, new_user: NewUser) -> Result<User> {
    let password_hash = hash_password(&new_user.password)?;

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, username, email, password_hash, created_at, updated_at
        "#,
        new_user.username,
        new_user.email,
        password_hash
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn get_user(pool: &PgPool, user_id: Uuid) -> Result<Option<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, password_hash, created_at, updated_at
        FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, password_hash, created_at, updated_at
        FROM users
        WHERE email = $1
        "#,
        email
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn list_users(pool: &PgPool) -> Result<Vec<User>> {
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, password_hash, created_at, updated_at
        FROM users
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

pub async fn update_user(pool: &PgPool, user_id: Uuid, update_user: UpdateUser) -> Result<User> {
    let mut tx = pool.begin().await?;

    let user_to_update = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, password_hash, created_at, updated_at
        FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| anyhow::anyhow!("User not found"))?;

    let username = update_user.username.unwrap_or(user_to_update.username);
    let email = update_user.email.unwrap_or(user_to_update.email);
    
    let password_hash = if let Some(password) = update_user.password {
        hash_password(&password)?
    } else {
        user_to_update.password_hash
    };

    let user = sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET username = $1, email = $2, password_hash = $3
        WHERE id = $4
        RETURNING id, username, email, password_hash, created_at, updated_at
        "#,
        username,
        email,
        password_hash,
        user_id
    )
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(user)
}

pub async fn delete_user(pool: &PgPool, user_id: Uuid) -> Result<()> {
    let result = sqlx::query!(
        r#"
        DELETE FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(anyhow::anyhow!("User not found"));
    }

    Ok(())
}
