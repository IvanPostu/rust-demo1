use std::{
    collections::{HashMap, HashSet},
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, LazyLock,
    },
};

use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{request::Parts, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Form, Json, Router,
};

#[derive(Debug)]
struct AppState {
    counter: AtomicU64,
}

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(AppState {
        counter: AtomicU64::new(0),
    });

    let greeting = "Hello!".to_string();

    let users_v1_router = Router::new().route("/users", get(list_users_v1));
    let users_v2_router = Router::new().route("/users", get(list_users_v2));

    let app = Router::new()
        .fallback(my_fallback)
        .nest("/api/v1", users_v1_router)
        .nest("/api/v2", users_v2_router)
        .route("/users", post(create_user))
        .route("/users2", post(create_user2))
        .route("/users3", post(create_user3))
        .route(
            "/greeting1/{name}/{example}",
            get(async move |path_args: Path<(String, String)>| {
                return format!("{} - {} - {}", greeting, path_args.0 .0, path_args.0 .1);
            }),
        )
        .route(
            "/greeting2/{name}/{example}",
            get(async move |Path((name, example)): Path<(String, String)>| {
                return format!("Test - {} - {}", name, example);
            }),
        )
        .route("/math/add/{arg1}/{arg2}", get(add))
        .route("/hello", get(hello))
        .route("/handler_1", get(handler_1))
        .route("/handler_2", get(handler_2))
        .route("/handler_3", get(handler_3))
        .route("/handler_4", get(handler_4))
        .route("/handler_5", get(handler_5))
        .route("/handler_6", get(handler_6))
        .with_state(shared_state);

    // pros: each router instance can have its own state
    let qusers_router = Router::new()
        .route("/qusers", get(list_users))
        .with_state(Arc::new(UserState {}));
    let qproducts_router = Router::new()
        .route("/qproducts", get(list_products))
        .with_state(Arc::new(ProductState {}));
    let app = app.merge(qusers_router).merge(qproducts_router);

    // Limitation: to create closure for handler function
    // fn make_hello_handler(greeting: String) -> impl AsyncFn() -> String {
    //     async move || format!("{}", greeting)
    // }
    // let greeting = "Hello!".to_string();
    // let closure = make_hello_handler(greeting);
    // let app = Router::new().route("/hello", get(closure));

    // GET & POST for same url
    // let app = Router::new().route("/api/users", get(list_users).post(create_user));
    // get, post, put, delete, patch, head, option и trace
    // any - for any http method

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug)]
struct UserState {}

#[derive(Debug)]
struct ProductState {}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    name: String,
}

#[allow(dead_code)]
#[derive(Serialize)]
struct CreateUserResponse {
    name: String,
}

async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateUserRequest>,
) -> impl IntoResponse {
    let prev_value = state.counter.fetch_add(1, Ordering::Relaxed);
    format!("Created user: {input:?}, value={}", prev_value + 1)
}

async fn create_user2(Form(input): Form<CreateUserRequest>) -> impl IntoResponse {
    format!("Created user: {input:?}")
}

async fn create_user3(Json(input): Json<CreateUserRequest>) -> Response {
    static USERS: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

    if input.name.is_empty() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("content-type", "text/plain; charset=utf-8")
            .body(Body::new("Empty name".to_string()))
            .unwrap();
    }
    let mut guard = USERS.lock().await;
    if guard.contains(&input.name) {
        Response::builder()
            .status(StatusCode::OK)
            .body(Body::empty())
            .unwrap()
    } else {
        guard.insert(input.name.clone());
        Response::builder()
            .status(StatusCode::CREATED)
            .header("content-type", "application/json; charset=utf-8")
            .body(Body::new(
                serde_json::to_string(&CreateUserResponse { name: input.name }).unwrap(),
            ))
            .unwrap()
    }

    // shorter version
    // if input.name.is_empty() {
    //     return (StatusCode::BAD_REQUEST, "Empty name").into_response();
    // }
    // let mut guard = USERS.lock().await;
    // if guard.contains(&input.name) {
    //     ().into_response()
    // } else {
    //     guard.insert(input.name.clone());
    //     let created_user = CreateUserResponse { name: input.name };
    //     (StatusCode::CREATED, Json(created_user)).into_response()
    // }
}

async fn add(Path((arg1, arg2)): Path<(i32, i32)>) -> String {
    format!("{arg1} + {arg2} = {}", arg1 + arg2)
}

async fn hello(headers: HeaderMap, Query(params): Query<HashMap<String, String>>) -> Response {
    let headers_string = headers
        .iter()
        .map(|(h, v)| format!("{}={}", h.as_str(), String::from_utf8_lossy(v.as_bytes())))
        .collect::<Vec<_>>()
        .join(",");

    // we can also use parts that has headers field
    // pub struct Parts {
    //     pub method: Method,
    //     pub uri: Uri,
    //     pub version: Version,
    //     pub headers: HeaderMap<HeaderValue>,
    //     pub extensions: Extensions,
    // }

    let content = match params.get("name") {
        Some(name) => format!("Hello, {}!\nHeaders: {}", name, headers_string),
        None => "Hello!".to_owned(),
    };
    let body = Body::new(content);
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/plain; charset=utf-8")
        .body(body)
        .unwrap()
}

async fn handler_1() -> StatusCode {
    StatusCode::OK
}

#[derive(serde::Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_2(Query(params): Query<HelloParams>) -> (StatusCode, String) {
    //  &'static str is also suitable
    let content = match params.name {
        Some(name) => format!("Hello, {}!", name),
        None => "Hello!".to_owned(),
    };
    (StatusCode::OK, content)
}

async fn handler_3() -> Result<String, (StatusCode, String)> {
    Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        "Some problem".to_string(),
    ))
}

async fn handler_4() -> Vec<u8> {
    vec![1, 2, 3]
}

use futures::lock::Mutex;
use serde_json::{json, Value};
async fn handler_5() -> Json<Value> {
    Json(json!({"name": "John Doe"}))
}

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
}
async fn handler_6() -> Json<Person> {
    Json(Person {
        name: "John Doe".to_string(),
    })
}

async fn my_fallback(parts: Parts) -> Response {
    let content = format!("Method: {}\nURL: {}", parts.method, parts.uri);
    (StatusCode::NOT_FOUND, content).into_response()
}

async fn list_users(State(user_state): State<Arc<UserState>>) -> String {
    format!("Users endpoint. UserState: {user_state:?}")
}

async fn list_products(State(state): State<Arc<ProductState>>) -> String {
    format!("Products endpoint. State: {state:?}")
}

async fn list_users_v1() -> &'static str {
    "Users endpoint Version 1"
}

async fn list_users_v2() -> &'static str {
    "Users endpoint Version 2"
}
