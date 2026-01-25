use bigdecimal::{BigDecimal, FromPrimitive};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .connect("postgres://postgres:1111@localhost/mydb")
        .await
        .unwrap();

    insert_accounts(&pool, "John Doe", BigDecimal::from_i32(1000).unwrap())
        .await
        .unwrap();

    let accounts = fetch_accounts(&pool).await.unwrap();
    println!("{accounts:?}");
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Account {
    id: i64,
    owner_name: String,
    balance: BigDecimal,
}

pub async fn fetch_accounts(db: &PgPool) -> Result<Vec<Account>, sqlx::Error> {
    sqlx::query_as("SELECT id, owner_name, balance FROM accounts")
        .fetch_all(db)
        .await
}

pub async fn insert_accounts(
    db: &PgPool,
    owner_name: &str,
    initial_balance: BigDecimal,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO accounts(owner_name, balance) VALUES($1, $2)")
        .bind(owner_name)
        .bind(initial_balance)
        .execute(db)
        .await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use sqlx::{
        migrate::Migrator,
        postgres::{PgConnectOptions, PgPoolOptions},
    };
    use testcontainers::{
        core::{IntoContainerPort, WaitFor},
        runners::AsyncRunner,
        GenericImage, ImageExt,
    };

    #[tokio::test]
    async fn test_create_account() {
        let container = GenericImage::new("postgres", "17-alpine")
            .with_wait_for(WaitFor::message_on_stderr(
                "database system is ready to accept connections",
            ))
            .with_exposed_port(5432.tcp())
            .with_env_var("POSTGRES_PASSWORD", "1111")
            .start()
            .await
            .expect("Postgres started");

        let connection_options = PgConnectOptions::new()
            .host(&container.get_host().await.unwrap().to_string())
            .port(container.get_host_port_ipv4(5432).await.unwrap())
            .database("postgres")
            .username("postgres")
            .password("1111");

        let pool = PgPoolOptions::new()
            .connect_with(connection_options)
            .await
            .unwrap();

        Migrator::new(std::path::Path::new("./migrations"))
            .await
            .unwrap()
            .run(&pool)
            .await
            .unwrap();

        insert_accounts(&pool, "Test-Account-1", BigDecimal::from_i32(1000).unwrap())
            .await
            .unwrap();

        let accounts = fetch_accounts(&pool).await.unwrap();

        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].owner_name, "Test-Account-1".to_string());
        assert_eq!(accounts[0].balance, BigDecimal::from_i32(1000).unwrap());
    }
}
