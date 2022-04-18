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
    use std::{borrow::Cow, future::Future, pin::Pin};

    use sqlx::{
        any::AnyRow,
        error::BoxDynError,
        migrate::{Migration, MigrationSource, MigrationType, Migrator},
        FromRow, Row,
    };

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

    #[tokio::test]
    async fn insert_test() -> anyhow::Result<()> {
        let pool = new_pool("sqlite://:memory:").await?;
        let mut connection = pool.acquire().await?;
        sqlx::query("CREATE TABLE tbl1(col1 INTEGER PRIMARY KEY, col2 INTEGER, col3 INTEGER)")
            .execute(&mut connection)
            .await?;
        sqlx::query("INSERT INTO tbl1(col1,col2,col3) VALUES (1, 2, 3)")
            .execute(&mut connection)
            .await?;
        let row = sqlx::query("SELECT col1, col2, col3 FROM tbl1 WHERE col1 = 1")
            .fetch_one(&mut connection)
            .await?;
        let col1: i64 = row.get("col1");
        let col2: i64 = row.get("col2");
        let col3: i64 = row.get("col3");
        assert_eq!(col1, 1);
        assert_eq!(col2, 2);
        assert_eq!(col3, 3);
        Ok(())
    }

    #[tokio::test]
    async fn custom_migration_source_test() -> anyhow::Result<()> {
        #[derive(Debug)]
        struct MyMigrationSource {}
        impl MigrationSource<'static> for MyMigrationSource {
            fn resolve(
                self,
            ) -> Pin<Box<dyn Future<Output = Result<Vec<Migration>, BoxDynError>> + Send + 'static>>
            {
                Box::pin(async move {
                    let migrations = vec![Migration::new(
                        20220223000001,
                        Cow::from("create table1"),
                        MigrationType::Simple,
                        Cow::from("CREATE TABLE table1 (col1 INTEGER PRIMARY KEY);"),
                    )];
                    Ok(migrations)
                })
            }
        }
        let pool = new_pool("sqlite://:memory:").await?;
        let migrator = Migrator::new(MyMigrationSource {}).await?;
        migrator.run(&pool).await?;
        let mut connection = pool.acquire().await?;
        let rows_affected = sqlx::query("INSERT INTO table1 VALUES (2)")
            .execute(&mut connection)
            .await?
            .rows_affected();
        assert_eq!(rows_affected, 1);
        let row = sqlx::query("SELECT col1 FROM table1")
            .fetch_one(&mut connection)
            .await?;
        let col1: i64 = row.get("col1");
        assert_eq!(col1, 2);

        // CREATE TABLE IF NOT EXISTS _sqlx_migrations (
        //     version BIGINT PRIMARY KEY,
        //     description TEXT NOT NULL,
        //     installed_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        //     success BOOLEAN NOT NULL,
        //     checksum BLOB NOT NULL,
        //     execution_time BIGINT NOT NULL
        // );
        #[derive(Debug)]
        struct MySqlxMigrationRow {
            version: i64,
            description: String,
            installed_on: String,
            success: bool,
            checksum: Vec<u8>,
            execution_time: i64,
        }
        impl<'r> FromRow<'r, AnyRow> for MySqlxMigrationRow {
            fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
                Ok(Self {
                    version: row.get("version"),
                    description: row.get("description"),
                    installed_on: row.get("installed_on"),
                    success: row.get("success"),
                    checksum: row.get("checksum"),
                    execution_time: row.get("execution_time"),
                })
            }
        }
        let row: MySqlxMigrationRow = sqlx::query_as("SELECT * FROM _sqlx_migrations")
            .fetch_one(&mut connection)
            .await?;
        assert_eq!(row.version, 20220223000001);
        assert_eq!(row.description, "create table1");
        assert_ne!(row.installed_on, "");
        assert!(row.success);
        assert!(!row.checksum.is_empty());
        assert_ne!(row.execution_time, 0);
        // println!("{:?}", row);
        Ok(())
    }
}
