use sqlx::PgPool;

pub struct User;

impl User {
    pub async fn cek_login(
        pool: &PgPool,
        username: &str,
        password: &str,
    ) -> Result<bool, sqlx::Error> {
        let user = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS (
                SELECT 1
                FROM users
                WHERE username = $1
                  AND password = $2
            )",
        )
        .bind(username)
        .bind(password)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
}
