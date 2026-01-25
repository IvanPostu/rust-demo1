#[tokio::main]
async fn main() {
    server::run_server().await
}

mod server {
    use std::sync::Arc;

    use crate::persist::{self, Account};
    use axum::{
        extract::State,
        http::StatusCode,
        routing::{get, post},
        Json, Router,
    };
    use serde::Deserialize;
    use sqlx::{postgres::PgPoolOptions, types::BigDecimal, PgPool};

    struct AppState {
        db: PgPool,
    }

    pub async fn run_server() {
        let pool = PgPoolOptions::new()
            .connect("postgres://postgres:1111@localhost/mydb")
            .await
            .unwrap();

        let state = AppState { db: pool };

        let app = Router::new()
            .route("/accounts", get(list_accounts))
            .route("/accounts", post(create_new_account))
            .with_state(Arc::new(state));

        let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

        axum::serve(listener, app).await.unwrap();
    }

    #[derive(Deserialize)]
    struct NewAcc {
        owner_name: String,
        init_balance: BigDecimal,
    }

    async fn list_accounts(
        state: State<Arc<AppState>>,
    ) -> Result<Json<Vec<Account>>, (StatusCode, String)> {
        match persist::fetch_accounts(&state.db).await {
            Ok(accounts) => Ok(Json(accounts)),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        }
    }

    async fn create_new_account(
        state: State<Arc<AppState>>,
        Json(acc): Json<NewAcc>,
    ) -> Result<Json<Account>, (StatusCode, String)> {
        match persist::create_accounts(&state.db, &acc.owner_name, acc.init_balance).await {
            Ok(account) => Ok(Json(account)),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        }
    }
}

mod persist {
    use bigdecimal::BigDecimal;
    use serde::{Deserialize, Serialize};
    use sqlx::{FromRow, PgPool};

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

    pub async fn create_accounts(
        db: &PgPool,
        owner_name: &str,
        initial_balance: BigDecimal,
    ) -> Result<Account, sqlx::Error> {
        let mut tx = db.begin().await?;
        sqlx::query("INSERT INTO accounts(owner_name, balance) VALUES($1, $2)")
            .bind(owner_name)
            .bind(initial_balance)
            .execute(&mut *tx)
            .await?;

        let result = sqlx::query_as(
            r#"
            SELECT id, owner_name, balance
            FROM accounts
            WHERE id = currval('accounts_seq')
        "#,
        )
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(result)
    }
}
