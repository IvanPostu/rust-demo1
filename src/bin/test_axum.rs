use axum::extract::FromRequest;
use axum::http::Method;
use axum::middleware::from_fn;
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::{
    body::{to_bytes, Body, Bytes},
    routing::post,
};
use axum::{
    extract::{FromRequestParts, Path, Query},
    http::{request::Parts, HeaderMap},
    Form, Json,
};
use axum::{
    extract::{Request, State},
    http::StatusCode,
    routing::get,
    Router,
};
use serde::Serialize;
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::{json, Value};
use tower_http::cors::CorsLayer;
use std::io::Cursor;
use std::{collections::HashMap, sync::Arc};
use std::{
    collections::HashSet,
    sync::{
        atomic::{AtomicU64, Ordering},
        LazyLock,
    },
};
use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::sync::{Mutex, RwLock};
use tokio::time::Instant;
use tower::{Layer, Service};
use tower_http::services::ServeFile;

tokio::task_local! {
    pub static SESSION: Arc<Mutex<SessionData>>;
}

struct SessionData {
    user_name: String,
}

struct AppState {
    counter: AtomicU64,
    sessions: RwLock<HashMap<String, Arc<Mutex<SessionData>>>>,
}

struct Session(String, Arc<Mutex<SessionData>>);

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let allowed_origins = [
        "http://mydomain.com".parse().unwrap(),
        "http://api.mydomain.com".parse().unwrap(),
    ];

    let sessions: RwLock<HashMap<String, Arc<Mutex<SessionData>>>> = {
        let mut data = HashMap::new();
        data.insert(
            "1111-1111-1111".to_string(),
            Arc::new(Mutex::new(SessionData {
                user_name: "John Doe".to_string(),
            })),
        );
        RwLock::new(data)
    };
    let shared_state = Arc::new(AppState {
        counter: AtomicU64::new(0),
        sessions,
    });

    let greeting = "Hello!".to_string();

    let users_v1_router = Router::new().route("/users", get(list_users_v1));
    let users_v2_router = Router::new().route("/users", get(list_users_v2));

    let app = Router::new()
        .layer(CorsLayer::new().allow_origin(allowed_origins))
        .fallback(my_fallback)
        .nest("/api/v1", users_v1_router)
        .nest("/api/v2", users_v2_router)
        .route("/users", post(create_user))
        .route("/users2", post(create_user2))
        .route("/users3", post(create_user3))
        .route("/users4", post(create_user4))
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
        .route("/hello2", get(hello2))
        .route_service(
            "/hello3",
            HelloService {
                greeting: "Hello!".to_string(),
            },
        )
        .route("/handler_1", get(handler_1))
        .route("/handler_2", get(handler_2))
        .route("/handler_3", get(handler_3))
        .route("/handler_4", get(handler_4))
        .route("/handler_5", get(handler_5))
        .route("/handler_6", get(handler_6))
        .route_service("/index", ServeFile::new("index.html"))
        .with_state(shared_state.clone())
        .layer(middleware::from_fn(log_exec_time))
        .layer(ExecTimeLogLayer)
        .layer(from_fn(async |request: Request, next: Next| {
            tracing::info!("Middleware-1: before call");
            let response = next.run(request).await;
            tracing::info!("Middleware-1: after call");
            response
        }))
        .layer(from_fn(async |request: Request, next: Next| {
            tracing::info!("Middleware-2: before call");
            let response = next.run(request).await;
            tracing::info!("Middleware-2: after call");
            response
        }))
        .layer(middleware::from_fn_with_state(
            shared_state,
            set_session_for_request,
        )); // last middleware is first inside the chain

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

#[derive(Clone)]
struct ExecTimeLogService<S> {
    next_handler: S,
}

impl<S> Service<Request> for ExecTimeLogService<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.next_handler.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let start = Instant::now();
        let future = self.next_handler.call(request);
        Box::pin(async move {
            let response = future.await?;
            tracing::info!(
                "Custom: Request took: {} micros",
                start.elapsed().as_micros()
            );
            Ok(response)
        })
    }
}

#[derive(Clone)]
struct ExecTimeLogLayer;

impl<S> Layer<S> for ExecTimeLogLayer {
    type Service = ExecTimeLogService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ExecTimeLogService {
            next_handler: inner,
        }
    }
}

#[derive(Clone)]
struct HelloService {
    greeting: String,
}

impl Service<Request> for HelloService {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request) -> Self::Future {
        if req.method() == Method::GET {
            let greeting = self.greeting.clone();
            Box::pin(async move { Ok(greeting.into_response()) })
        } else {
            Box::pin(async move { Ok(StatusCode::METHOD_NOT_ALLOWED.into_response()) })
        }
    }
}

async fn set_session_for_request(
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Response {
    let session = if let Some(value) = request.headers().get("sessionid") {
        if let Ok(string_value) = value.to_str() {
            let session_id = string_value.to_string();
            let read_guard = state.sessions.read().await;
            if let Some(session) = read_guard.get(&session_id) {
                session.clone()
            } else {
                return StatusCode::UNAUTHORIZED.into_response();
            }
        } else {
            return StatusCode::BAD_REQUEST.into_response();
        }
    } else {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    let response = SESSION
        .scope(session, async { next.run(request).await })
        .await;
    response
}

async fn log_exec_time(request: Request, next: Next) -> Response {
    let start = Instant::now();
    let response = next.run(request).await;
    tracing::info!(
        "Original: Request took: {} micros",
        start.elapsed().as_micros()
    );
    response
}

struct AnyFormat<D: DeserializeOwned>(D);

impl<S: Send + Sync, D: DeserializeOwned> FromRequest<S> for AnyFormat<D> {
    type Rejection = (StatusCode, &'static str);

    async fn from_request(request: Request<Body>, _state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = request.into_parts();
        let Some(content_type) = parts.headers.get("content-type") else {
            return Err((StatusCode::BAD_REQUEST, "Missing content-type"));
        };
        let body_bytes: Bytes = to_bytes(body, 100 * 1024 * 1024).await.unwrap();

        let result = match content_type.to_str().unwrap() {
            "application/json" => match serde_json::from_slice::<D>(&body_bytes) {
                Ok(entity) => entity,
                Err(_) => return Err((StatusCode::BAD_REQUEST, "Malformed JSON")),
            },
            "application/xml" => {
                let cursor = Cursor::new(body_bytes);
                match serde_xml_rs::from_reader::<'_, D, _>(cursor) {
                    Ok(entity) => entity,
                    Err(_) => return Err((StatusCode::BAD_REQUEST, "Malformed XML")),
                }
            }
            _ => return Err((StatusCode::BAD_REQUEST, "Unsuported format")),
        };
        Ok(AnyFormat(result))
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct CreateUserRequest4 {
    name: String,
}

async fn create_user4(AnyFormat(req): AnyFormat<CreateUserRequest4>) -> String {
    tracing::info!("calling create_user4");
    format!("Received: {req:?}")
}

impl FromRequestParts<Arc<AppState>> for Session {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        if let Some(value) = parts.headers.get("sessionid") {
            if let Ok(string_value) = value.to_str() {
                let session_id = string_value.to_string();
                let read_guard = state.sessions.read().await;
                if let Some(session) = read_guard.get(&session_id) {
                    Ok(Session(session_id, session.clone()))
                } else {
                    Err(StatusCode::UNAUTHORIZED)
                }
            } else {
                Err(StatusCode::BAD_REQUEST)
            }
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

struct SessionId(String);

impl<S: Send + Sync> FromRequestParts<S> for SessionId {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(value) = parts.headers.get("sessionid") {
            if let Ok(string_value) = value.to_str() {
                Ok(SessionId(string_value.to_string()))
            } else {
                Err(StatusCode::BAD_REQUEST)
            }
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

struct MyQueryParams(HashMap<String, String>);

impl<S: Send + Sync> FromRequestParts<S> for MyQueryParams {
    type Rejection = ();

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let mut params = HashMap::new();
        if let Some(query_string) = parts.uri.query() {
            for pair in query_string.split("&") {
                let mut kv = pair.split("=");
                if let Some(k) = kv.next() {
                    if let Some(v) = kv.next() {
                        params.insert(k.to_string(), v.to_string());
                    }
                }
            }
        }
        Ok(MyQueryParams(params))
    }
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

async fn hello2(
    MyQueryParams(map): MyQueryParams,
    SessionId(session_id): SessionId,
    Session(id, session): Session,
) -> String {
    format!(
        "Query params: {map:?}, Session ID: {session_id}, Session ID: {id}, User name: {}",
        session.lock().await.user_name,
    )
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

    let session = SESSION.with(|session| session.clone());

    let content = match params.get("name") {
        Some(name) => format!(
            "Hello, {}!\nHeaders: {}, task local user: {}",
            name,
            headers_string,
            session.lock().await.user_name
        ),
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

async fn handler_5() -> Json<Value> {
    Json(json!({"name": "John Doe"}))
}

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
