use sqlx::sqlite::SqlitePoolOptions;

#[derive(Debug, Eq, PartialEq, sqlx::FromRow)]
struct User {
    id: i32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite://test.sqlite?mode=rwc")
        .await?;

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
    println!("{:?}", user);

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);
    println!("{:?}", row);

    Ok(())
}
