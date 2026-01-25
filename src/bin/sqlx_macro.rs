#[allow(unused_imports)]
use sqlx::{
    postgres::{PgPoolOptions, PgQueryResult},
    types::BigDecimal,
};

#[derive(Debug)]
struct Account {
    id: i64,
    owner_name: String,
    balance: BigDecimal,
}

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .connect("postgres://postgres:1111@localhost/mydb")
        .await
        .unwrap();

    {
        let all_accounts: Vec<Account> =
            sqlx::query_as!(Account, "SELECT id, owner_name, balance FROM accounts")
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

        let opt_acc_1: Option<Account> = sqlx::query_as!(
            Account,
            "SELECT id, owner_name, balance FROM accounts WHERE owner_name=$1",
            "John Doe" // Значение для аргумента $1
        )
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
    }

    {
        let account_ids: Vec<i64> = sqlx::query_scalar!("SELECT id FROM accounts")
            .fetch_all(&pool)
            .await
            .unwrap();
        println!("All IDs: {account_ids:?}");

        let accounts_count: Option<i64> = sqlx::query_scalar!("SELECT COUNT(*) FROM accounts")
            .fetch_one(&pool)
            .await
            .unwrap();
        println!("Number of accounts: {}", accounts_count.unwrap_or_default());
    }

    // change owner_name on each run
    // {
    //     let result: PgQueryResult = sqlx::query!(
    //         "INSERT INTO accounts(owner_name, balance) VALUES($1, $2)",
    //         "Some Name1",
    //         BigDecimal::from(42),
    //     )
    //     .execute(&pool)
    //     .await
    //     .unwrap();
    //     println!("{result:?}"); // PgQueryResult { rows_affected: 1 }
    // }
}
