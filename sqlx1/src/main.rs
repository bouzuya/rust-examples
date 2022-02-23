use std::str::FromStr;

use sqlx::{any::AnyConnectOptions, AnyPool};

fn main() {
    println!("Hello, world!");
}

pub async fn new_pool(url: &str) -> Result<AnyPool, sqlx::Error> {
    let options = AnyConnectOptions::from_str(url)?;
    AnyPool::connect_with(options).await
}

#[cfg(test)]
mod tests {
    use sqlx::{any::AnyRow, FromRow, Row};

    use super::*;

    #[tokio::test]
    async fn query_fetch_one_test() -> anyhow::Result<()> {
        let pool = new_pool("sqlite://:memory:").await?;
        let mut connection = pool.acquire().await?;
        let row = sqlx::query("SELECT 1 AS col1, 'ABC' AS col2")
            .fetch_one(&mut connection)
            .await?;
        let col1: i64 = row.get(0);
        assert_eq!(col1, 1);
        let col1: i64 = row.get("col1");
        assert_eq!(col1, 1);
        let col2: String = row.get(1);
        assert_eq!(col2, "ABC");
        let col2: String = row.get("col2");
        assert_eq!(col2, "ABC");
        Ok(())
    }

    #[tokio::test]
    async fn query_as_fetch_one_test() -> anyhow::Result<()> {
        struct MyRow {
            col1: i64,
            col2: String,
        }
        impl<'r> FromRow<'r, AnyRow> for MyRow {
            fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
                Ok(Self {
                    col1: row.get("col1"),
                    col2: row.get("col2"),
                })
            }
        }
        let pool = new_pool("sqlite://:memory:").await?;
        let mut connection = pool.acquire().await?;
        let my_row: MyRow = sqlx::query_as("SELECT 1 AS col1, 'ABC' AS col2")
            .fetch_one(&mut connection)
            .await?;
        assert_eq!(my_row.col1, 1);
        assert_eq!(my_row.col2, "ABC");
        Ok(())
    }
}
