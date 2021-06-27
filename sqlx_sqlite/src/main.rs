fn main() {}

#[cfg(test)]
mod tests {
    use anyhow::Context as _;
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::{fs, path::PathBuf, str::FromStr};

    #[derive(Debug, Eq, PartialEq, sqlx::FromRow)]
    struct User {
        id: i32,
    }

    #[test]
    fn connect_options_test() -> anyhow::Result<()> {
        // :memory: == sqlite::memory == sqlite://:memory:
        SqliteConnectOptions::from_str(":memory:")?;
        SqliteConnectOptions::from_str("sqlite::memory:")?;
        SqliteConnectOptions::from_str("sqlite://:memory:")?;
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn connection_test() -> anyhow::Result<()> {
        let pool = SqlitePoolOptions::new().connect(":memory:").await?;
        let row: (i64,) = sqlx::query_as("SELECT $1")
            .bind(150_i64)
            .fetch_one(&pool)
            .await?;
        assert_eq!(row.0, 150);
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test() -> anyhow::Result<()> {
        let path_buf: PathBuf = "test.sqlite".parse()?;
        if path_buf.exists() {
            fs::remove_file(path_buf.as_path())?;
        }
        let uri = format!(
            "sqlite://{}?mode=rwc",
            path_buf.to_str().context("no utf-8")?
        );
        let pool = SqlitePoolOptions::new().connect(uri.as_str()).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY)")
            .execute(&pool)
            .await?;
        sqlx::query("INSERT INTO users VALUES (?)")
            .bind(1_i64)
            .execute(&pool)
            .await?;
        let user: User = sqlx::query_as::<_, User>("SELECT id FROM users")
            .fetch_one(&pool)
            .await?;

        assert_eq!(user, User { id: 1 });

        let row: (i64,) = sqlx::query_as("SELECT $1")
            .bind(150_i64)
            .fetch_one(&pool)
            .await?;

        assert_eq!(row.0, 150);

        Ok(())
    }
}