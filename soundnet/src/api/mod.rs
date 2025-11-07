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
use tracing::{info, warn};

pub async fn run(app_state: Arc<Mutex<AppState>>) -> Result<(), anyhow::Error> {
    let app = Router::new()
        .route("/api/v1/status", get(get_status))
        .route("/api/v1/mode", post(set_mode))
        .route("/api/v1/volume", put(set_volume))
        .route("/api/v1/stream/format", put(set_stream_format))
        .with_state(app_state.clone());

    let addr = {
        let app_state = app_state.lock().unwrap();
        SocketAddr::from(([127, 0, 0, 1], app_state.config.api_port))
    };
    info!("Starting API server on {}", addr);
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
    info!("Setting mode to {:?}", payload.mode);
    let mut app_state = app_state.lock().unwrap();
    let mode = match payload.mode {
        DeviceMode::Server => crate::Mode::Server,
        DeviceMode::Client => crate::Mode::Client { jitter_buffer_size: None },
        DeviceMode::Idle => {
            app_state.stop_tasks();
            return;
        }
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
    info!("Setting volume to {}", payload.volume);
    let app_state = app_state.lock().unwrap();
    let mut state = app_state.state.lock().unwrap();
    if state.mode == DeviceMode::Client {
        state.format.volume = payload.volume;
    } else {
        warn!("Cannot set volume when not in client mode");
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
    info!("Setting stream format to {:?}", payload.format);
    let app_state = app_state.lock().unwrap();
    let mut state = app_state.state.lock().unwrap();
    if state.mode == DeviceMode::Server {
        state.format = payload.format;
    } else {
        warn!("Cannot set stream format when not in server mode");
    }
}