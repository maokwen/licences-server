use axum::{
    extract::{Json, Path, State},
    http::{header, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Entry {
    id: Option<i32>,
    key: Option<String>,
    license: Option<String>,
}

struct AppState {
    db: Mutex<Connection>,
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers(vec![header::CONTENT_TYPE])
        .allow_origin(tower_http::cors::Any);

    let shared_state = Arc::new(AppState {
        db: Mutex::new(Connection::open("licenses.sqlite3").unwrap()),
    });

    // our router
    let app = Router::new()
        .route("/admin/list", get(get_list))
        .route("/:key", get(get_key))
        .route("/admin/add", post(post_add))
        .route("/admin/remove", post(post_remove))
        .route("/admin/update", post(post_update))
        .with_state(shared_state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/*****************
 * Error Handlers
 *****************/

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

/*****************
 * Route Handlers
 *****************/

async fn get_list(State(state): State<Arc<AppState>>) -> Result<Response, AppError> {
    let db = state.db.lock().await;

    let mut stmt = db.prepare("SELECT id, key, license FROM licenses")?;
    let rows = stmt.query_map([], |row| {
        Ok(Entry {
            id: row.get(0)?,
            key: row.get(1)?,
            license: row.get(2)?,
        })
    })?;

    let mut entries = Vec::new();
    for r in rows {
        entries.push(r?);
    }

    Ok(Json(entries).into_response())
}

async fn get_key(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
) -> Result<Response, AppError> {
    let db = state.db.lock().await;

    let mut stmt = db.prepare("SELECT id, key, license FROM licenses WHERE key = ?")?;
    let rows = stmt.query_map([key], |row| {
        Ok(Entry {
            id: row.get(0)?,
            key: row.get(1)?,
            license: row.get(2)?,
        })
    })?;

    for r in rows {
        if let Some(l) = r?.license {
            return Ok((StatusCode::OK, l).into_response());
        }
    }

    Ok((StatusCode::OK, "").into_response())
}

async fn post_add(
    State(state): State<Arc<AppState>>,
    Json(entry): Json<Entry>,
) -> Result<Response, AppError> {
    let db = state.db.lock().await;

    db.execute(
        "INSERT INTO licenses (key, license) VALUES (?, ?)",
        [entry.key, entry.license],
    )?;

    Ok((StatusCode::OK, "OK").into_response())
}

async fn post_remove(
    State(state): State<Arc<AppState>>,
    Json(entry): Json<Entry>,
) -> Result<Response, AppError> {
    let db = state.db.lock().await;

    db.execute("DELETE FROM licenses WHERE key = ?", [entry.key])?;

    Ok((StatusCode::OK, "OK").into_response())
}

async fn post_update(
    State(state): State<Arc<AppState>>,
    Json(entry): Json<Entry>,
) -> Result<Response, AppError> {
    let db = state.db.lock().await;

    db.execute(
        "UPDATE licenses SET license = ? WHERE key = ?",
        [entry.license, entry.key],
    )?;

    Ok((StatusCode::OK, "OK").into_response())
}
