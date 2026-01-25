use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::NaiveDateTime;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgQueryResult, PgRow},
    prelude::FromRow,
    PgPool, Pool, Postgres, QueryBuilder, Row,
};

#[derive(Debug, FromRow)]
struct Account {
    id: i64,
    owner_name: String,
    balance: BigDecimal,
}

#[derive(Debug, FromRow)]
struct TransactionFullInfo {
    id: i64,
    amount: BigDecimal,
    src_account_owner_name: String,
    dst_account_owner_name: String,
    tx_timestamp: NaiveDateTime,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let pool = PgPoolOptions::new()
        .connect("postgres://postgres:1111@localhost/mydb")
        .await
        .unwrap();

    {
        let all_accounts: Vec<Account> =
            sqlx::query_as("SELECT id, owner_name, balance FROM accounts")
                .fetch_all(&pool)
                .await
                .unwrap();
        for acc in all_accounts {
            println!(
                "{}: {}, {}",
                acc.id,
                acc.owner_name,
                acc.balance.to_string()
            );
        }
        // 1: John Doe, 1000
        // 2: Ivan Ivanov, 2000
    }

    {
        let opt_acc_1: Option<Account> = sqlx::query_as(
            r#"
        SELECT id, owner_name, balance FROM accounts WHERE owner_name=$1
    "#,
        )
        .bind("John Doe") // Привязываем значение к плэйсхолдеру $1
        .fetch_optional(&pool)
        .await
        .unwrap();
        if let Some(acc) = opt_acc_1 {
            println!(
                "{}: {}, {}",
                acc.id,
                acc.owner_name,
                acc.balance.to_string()
            );
        }
        // 1: John Doe, 1000
    }

    {
        let account_ids: Vec<i64> = sqlx::query_scalar("SELECT id FROM accounts")
            .fetch_all(&pool)
            .await
            .unwrap();
        println!("All IDs: {account_ids:?}"); // All IDs: [1, 2]

        let accounts_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM accounts")
            .fetch_one(&pool)
            .await
            .unwrap();
        println!("Number of accounts: {accounts_count}"); // Number of accounts: 2

        let transaction: Vec<TransactionFullInfo> = sqlx::query_as(
            r#"
        SELECT
            tx.id as id, tx.amount, tx.tx_timestamp,
            src_acc.owner_name as src_account_owner_name,
            dst_acc.owner_name as dst_account_owner_name
        FROM
            transactions tx
            JOIN accounts src_acc ON tx.src_account_id = src_acc.id
            JOIN accounts dst_acc ON tx.dst_account_id = dst_acc.id
    "#,
        )
        .fetch_all(&pool)
        .await
        .unwrap();

        for tx in transaction {
            println!(
                "TXID:{} amount={}, timestamp={}, src: {}, dst: {}",
                tx.id,
                tx.amount,
                tx.tx_timestamp,
                tx.src_account_owner_name,
                tx.dst_account_owner_name
            );
        }
        // TXID:1 amount=10, timestamp=2025-12-11 14:00:00, src: John Doe, dst: Ivan Ivanov
        // TXID:2 amount=20, timestamp=2025-12-12 15:00:00, src: Ivan Ivanov, dst: John Doe
    }

    {
        let total_amount: BigDecimal = sqlx::query_scalar(
            r#"
            SELECT
                SUM(amount)
            FROM transactions tx
                JOIN accounts src_acc ON tx.src_account_id = src_acc.id
                JOIN accounts dst_acc ON tx.dst_account_id = dst_acc.id
            WHERE
                amount >= $1
                AND src_acc.owner_name = $2
                AND dst_acc.owner_name = $3
        "#,
        )
        .bind(20.0)
        .bind("Ivan Ivanov")
        .bind("John Doe")
        .fetch_one(&pool)
        .await
        .unwrap();

        println!("{total_amount}"); // 20
    }

    {
        let rows: Vec<PgRow> = sqlx::query(
            r#"
            SELECT
                tx.id as id, tx.amount, tx.tx_timestamp,
                src_acc.owner_name as src_account_owner_name,
                dst_acc.owner_name as dst_account_owner_name
            FROM
                transactions tx
                JOIN accounts src_acc ON tx.src_account_id = src_acc.id
                JOIN accounts dst_acc ON tx.dst_account_id = dst_acc.id
        "#,
        )
        .fetch_all(&pool)
        .await
        .unwrap();

        for r in rows {
            let id: i64 = r.try_get("id").unwrap();
            let amount: BigDecimal = r.try_get("amount").unwrap();
            let ts: NaiveDateTime = r.try_get("tx_timestamp").unwrap();
            let src: String = r.try_get("src_account_owner_name").unwrap();
            let dst: String = r.try_get("dst_account_owner_name").unwrap();

            println!("TXID:{id} amount={amount}, timestamp={ts}, src: {src}, dst: {dst}");
        }
    }

    {
        // _insert_example().await;
        // _insert_multiple_accounts_via_query_builder().await;
    }

    {
        // transaction demo
        // let mut tx = pool.begin().await.expect("Cannot start transaction");

        // sqlx::query("UPDATE ...").execute(&mut *tx).await?;
        // sqlx::query("INSERT ...").execute(&mut *tx).await?;
        // sqlx::query("DELETE ...").execute(&mut *tx).await?;

        // tx.commit().await.expect("Cannot commit");
    }

    {
        let _transfer = Transfer {
            src_account_id: 1,
            dst_account_id: 2,
            amount: BigDecimal::from_f64(50.0).unwrap(),
        };

        // let _ = _make_transfer(&transfer, &pool).await.unwrap();

        let _transfer2 = Transfer {
            src_account_id: 1,
            dst_account_id: 2,
            amount: BigDecimal::from_f64(5000.0).unwrap(), // Account doesn't have so much money
        };

        if let Err(e) = _make_transfer(&_transfer2, &pool).await {
            // it rollbacks transaction
            tracing::error!(error = %e, "failed to make transfer");
        }
    }
}

#[allow(dead_code)]
struct Transfer {
    src_account_id: i64,
    dst_account_id: i64,
    amount: BigDecimal,
}

async fn _make_transfer(transfer: &Transfer, pool: &PgPool) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    let _ = sqlx::query("UPDATE accounts SET balance = balance + $1 WHERE id = $2")
        .bind(&transfer.amount)
        .bind(&transfer.dst_account_id)
        .execute(&mut *tx)
        .await?;

    let _ = sqlx::query("UPDATE accounts SET balance = balance - $1 WHERE id = $2")
        .bind(&transfer.amount)
        .bind(&transfer.src_account_id)
        .execute(&mut *tx)
        .await?;

    let _ = sqlx::query(
        r#"
            INSERT INTO transactions(
                amount, src_account_id, dst_account_id, tx_timestamp
            ) VALUES ($1, $2, $3, NOW())
        "#,
    )
    .bind(&transfer.amount)
    .bind(&transfer.src_account_id)
    .bind(&transfer.dst_account_id)
    .execute(&mut *tx)
    .await?;

    // Закрепляем транзакцию
    tx.commit().await?;

    Ok(())
}

#[allow(dead_code)]
struct NewAcc {
    owner_name: String,
    balance: BigDecimal,
}

async fn _insert_multiple_accounts_via_query_builder() {
    let pool = PgPoolOptions::new()
        .connect("postgres://postgres:1111@localhost/mydb")
        .await
        .unwrap();

    let new_accounts = vec![
        NewAcc {
            owner_name: "Name 1".to_string(),
            balance: BigDecimal::default(),
        },
        NewAcc {
            owner_name: "Name 2".to_string(),
            balance: BigDecimal::default(),
        },
        NewAcc {
            owner_name: "Name 3".to_string(),
            balance: BigDecimal::default(),
        },
    ];

    let mut qb: QueryBuilder<'_, Postgres> =
        QueryBuilder::new(r#"INSERT INTO accounts(owner_name, balance)"#);

    qb.push_values(&new_accounts, |mut builder, acc| {
        builder.push_bind(&acc.owner_name).push_bind(&acc.balance);
    });

    let result: PgQueryResult = qb.build().execute(&pool).await.unwrap();
    println!("{result:?}"); // PgQueryResult { rows_affected: 3 }
}

async fn _insert_example() {
    let pool = PgPoolOptions::new()
        .connect("postgres://postgres:1111@localhost/mydb")
        .await
        .unwrap();

    let result: PgQueryResult =
        sqlx::query("INSERT INTO accounts(owner_name, balance) VALUES($1, $2)")
            .bind("Some Name")
            .bind(BigDecimal::default())
            .execute(&pool)
            .await
            .unwrap();
    println!("{result:?}"); // PgQueryResult { rows_affected: 1 }
}

async fn _create_pool_via_connect() -> Pool<Postgres> {
    let pool: Pool<Postgres> = PgPoolOptions::new()
        // PostgreSQL: postgres://mylogin:mypassword@localhost/mydb\
        // MySQL: mysql://mylogin:mypassword@host/mydb\
        // SQLite: sqlite::memory: или sqlite://my.db
        .connect("postgres://postgres:1111@localhost/mydb")
        .await
        .unwrap();

    return pool;
}

async fn _create_pool_via_option() -> Pool<Postgres> {
    let connection_option = PgConnectOptions::new()
        .host("localhost")
        .username("postgres")
        .password("1111")
        .database("mydb");

    let pool = PgPoolOptions::new()
        .connect_with(connection_option)
        .await
        .unwrap();

    pool
}
