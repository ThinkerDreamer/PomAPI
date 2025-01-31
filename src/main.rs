use std::{collections::BTreeMap, net::SocketAddr, sync::Arc, time::Duration};

use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Serialize)]
struct Timer {
    id: Uuid,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
struct QuoteResponse {
    quote: &'static str,
}

#[derive(Debug, Serialize)]
struct StatusResponse {
    seconds: i64,
    minutes: i64,
    hours: i64,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let timers = Arc::new(Mutex::new(BTreeMap::<Uuid, Timer>::new()));

    let app = Router::new()
        .route("/quote", get(quote_handler))
        .route("/timer/{duration_in_min}", get(timer_handler))
        .route("/status/{id}", get(status_handler))
        .with_state(timers);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn quote_handler() -> Json<QuoteResponse> {
    Json(QuoteResponse {
        quote: "You can do it!",
    })
}

async fn timer_handler(
    Path(duration_in_min): Path<u64>,
    State(timers): State<Arc<Mutex<BTreeMap<Uuid, Timer>>>>,
) -> Json<Timer> {
    let duration = Duration::from_secs(60 * duration_in_min);
    let start = Utc::now();
    let end = start + duration;
    let id = Uuid::new_v4();
    let timer = Timer { start, end, id };

    timers.lock().await.insert(id, timer);

    Json(Timer { id, start, end })
}

async fn status_handler(
    Path(timer_id): Path<Uuid>,
    State(timers): State<Arc<Mutex<BTreeMap<Uuid, Timer>>>>,
) -> Result<Json<StatusResponse>, Json<&'static str>> {
    timers
        .lock()
        .await
        .get(&timer_id)
        .map(|timer| {
            let delta = timer.end - Utc::now();
            Json(StatusResponse {
                seconds: delta.num_seconds(),
                minutes: delta.num_minutes(),
                hours: delta.num_hours(),
            })
        })
        .ok_or(Json("Timer does not exist"))
}
