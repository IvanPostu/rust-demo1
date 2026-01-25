use axum::{
    extract::{FromRequestParts, Path, Query},
    http::StatusCode,
};
use std::collections::HashMap;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

const APIKEY_HEADER: &str = "x-key";

struct MySecurityAddon;

impl Modify for MySecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "apikey_auth",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new(APIKEY_HEADER))),
            )
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    info(description = "My Api description", license(name = "GPL 2")),
    modifiers(&MySecurityAddon), 
)]
struct MyApiDoc;

#[utoipa::path(
    get,
    path = "/hello",
    responses((status = 200, description = "'Hello' response", body = &'static str)),
    summary = "A hello endpoint",
    security( ("apikey_auth" = []) )
)]
async fn hello(_session: Authenticated) -> &'static str {
    "Hello"
}

struct Authenticated;
impl<S: Send + Sync> FromRequestParts<S> for Authenticated {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        if let Some(_session_id) = parts.headers.get(APIKEY_HEADER) {
            Ok(Authenticated)
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

#[utoipa::path(
    get,
    path = "/greet/{name}",
    params(
        ("greeting" = Option<String>, Query, description = "Greeting str, default: 'Hello'"),
    ),
    responses(
        (status = 200, description = "Greet response", body = String),
        (status = 400, description = "Wrong name error", body = String)
    ),
    summary = "A greet endpoint",
)]
async fn greet(
    Path(name): Path<String>,
    Query(map): Query<HashMap<String, String>>,
) -> (StatusCode, String) {
    if name.len() < 2 {
        return (StatusCode::BAD_REQUEST, "Wrong name".to_string());
    }
    let greeting = map.get("greeting").map(|s| s.as_str()).unwrap_or("Hello");
    (StatusCode::OK, format!("{greeting}, {name}!"))
}

#[tokio::main]
async fn main() {
    let open_api = OpenApiRouter::with_openapi(MyApiDoc::openapi()).nest(
        "/api",
        OpenApiRouter::new()
            .routes(routes!(greet))
            .routes(routes!(hello)),
    );

    let (router, api) = open_api.split_for_parts();

    // let app = router.route("/api-doc", routing::get(async move || api.to_pretty_json().unwrap()));

    let app = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
        .merge(Redoc::with_url("/redoc", api.clone()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .merge(Scalar::with_url("/scalar", api));

    // SwaggerUI: http://localhost:8080/swagger-ui/
    // Redoc: http://localhost:8080/redoc
    // RapiDoc: http://localhost:8080/rapidoc
    // Scalar: http://localhost:8080/scalar

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
