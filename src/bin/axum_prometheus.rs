use axum::{
    extract::{Request, State},
    middleware::{from_fn, Next},
    Router,
};
use axum::{response::Response, routing::get};
use metrics_process::Collector;
use metrics_prometheus::Recorder;
use prometheus::{Histogram, HistogramOpts};
use std::{sync::Arc, time::Duration};
use tokio::time::Instant;

struct AppState {
    recorder: Recorder,
}

#[tokio::main]
async fn main() {
    let recorder = metrics_prometheus::install();

    let state = AppState { recorder };

    let custom_buckets = vec![0.1, 0.25, 0.5, 0.9, 1.0];
    let opts = HistogramOpts::new("call_duration1", "My description").buckets(custom_buckets);
    let histogram = Histogram::with_opts(opts).unwrap();
    state.recorder.register_metric(histogram);

    for _ in 0..10 {
        let latency = Duration::from_millis(rand::random_range(0..1000));
        metrics::histogram!("call_duration1").record(latency.as_secs_f64());
    }

    metrics::gauge!("my_gauge").set(5);
    metrics::gauge!("my_gauge").increment(3);
    metrics::gauge!("my_gauge").decrement(1);
    metrics::gauge!("my_gauge").set(99);

    let report = prometheus::TextEncoder::new()
        .encode_to_string(&state.recorder.registry().gather())
        .unwrap();

    println!("{report}");

    let app = Router::new()
        .merge(
            Router::new()
                .route("/endpoint-1", get(endpoint_1))
                .route("/endpoint-2", get(endpoint_2))
                .layer(from_fn(metrics_middleware)),
        )
        .route("/metrics", get(get_metrics))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn metrics_middleware(request: Request, next: Next) -> Response {
    let path = request.uri().path().to_string();
    metrics::counter!("number_of_calls", "path" => path.clone()).increment(1);

    let start = Instant::now();
    let response = next.run(request).await;
    let time_in_seconds = start.elapsed().as_secs_f64();
    metrics::histogram!("call_duration", "path" => path).record(time_in_seconds);

    response
}

async fn endpoint_1() -> &'static str {
    tokio::time::sleep(Duration::from_millis(rand::random_range(0..1000))).await;
    "Endpoint 1"
}

async fn endpoint_2() -> &'static str {
    "Endpoint 2"
}

async fn get_metrics(state: State<Arc<AppState>>) -> String {
    let collector = Collector::default();
    collector.describe();
    collector.collect();
    
    let report = prometheus::TextEncoder::new()
        .encode_to_string(&state.recorder.registry().gather())
        .unwrap();

    report
}
