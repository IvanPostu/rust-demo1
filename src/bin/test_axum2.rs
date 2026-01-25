use axum::{extract::Query, routing::get, Router};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let app = setup_app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn setup_app() -> Router {
    Router::new().route("/hello", get(hello))
}

async fn hello(Query(map): Query<HashMap<String, String>>) -> String {
    if let Some(name) = map.get("name") {
        format!("Hello, {name}!")
    } else {
        String::from("Hello!")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::http::StatusCode;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_hello_endpoint() {
        let app = setup_app();

        let server = TestServer::new(app).unwrap();

        let response1 = server.get("/hello").await;
        response1.assert_status(StatusCode::OK);
        response1.assert_text("Hello!");

        let response2 = server.get("/hello?name=Test123").await;
        response2.assert_status(StatusCode::OK);
        response2.assert_text("Hello, Test123!");
    }
}
