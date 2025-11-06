use crate::AppState;
use axum::{
    extract::State,
    routing::{get, post, put},
    Json,
    Router,
};
use soundnet_types::{AudioFormat, DeviceMode};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

pub async fn run(app_state: Arc<Mutex<AppState>>) -> Result<(), anyhow::Error> {
    let app = Router::new()
        .route("/api/v1/status", get(get_status))
        .route("/api/v1/mode", post(set_mode))
        .route("/api/v1/volume", put(set_volume))
        .route("/api/v1/stream/format", put(set_stream_format))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_status(State(app_state): State<Arc<Mutex<AppState>>>) -> Json<DeviceMode> {
    let app_state = app_state.lock().unwrap();
    let state = app_state.state.lock().unwrap().mode.clone();
    Json(state)
}

#[derive(serde::Deserialize)]
pub struct SetModeRequest {
    pub mode: DeviceMode,
}

async fn set_mode(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<SetModeRequest>,
) {
    let mut app_state = app_state.lock().unwrap();
    let mode = match payload.mode {
        DeviceMode::Server => crate::Mode::Server,
        DeviceMode::Client => crate::Mode::Client { jitter_buffer_size: 20 }, // TODO: get from config
        DeviceMode::Idle => todo!(),
    };
    app_state.start_tasks(&mode);
}

#[derive(serde::Deserialize)]
pub struct SetVolumeRequest {
    pub volume: f32,
}

async fn set_volume(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<SetVolumeRequest>,
) {
    let app_state = app_state.lock().unwrap();
    let mut state = app_state.state.lock().unwrap();
    if state.mode == DeviceMode::Client {
        state.format.volume = payload.volume;
    }
}

#[derive(serde::Deserialize)]
pub struct SetStreamFormatRequest {
    pub format: AudioFormat,
}

async fn set_stream_format(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<SetStreamFormatRequest>,
) {
    let app_state = app_state.lock().unwrap();
    let mut state = app_state.state.lock().unwrap();
    if state.mode == DeviceMode::Server {
        state.format = payload.format;
    }
}