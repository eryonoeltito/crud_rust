use sqlx::PgPool;

pub struct User;

impl User {
    pub async fn register(
        pool: &PgPool,
        username: &str,
        password: &str,
    ) -> Result<bool, sqlx::Error> {
        let username_exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS (SELECT 1 FROM users WHERE username = $1)",
        )
        .bind(username)
        .fetch_one(pool)
        .await?;

        if username_exists {
            return Ok(false);
        }

        sqlx::query("INSERT INTO users (username, password) VALUES ($1, $2)")
            .bind(username)
            .bind(password)
            .execute(pool)
            .await?;

        Ok(true)
    }

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
