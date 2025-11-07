use crate::jitter_buffer::JitterBuffer;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam_channel::Receiver;
use soundnet_types::SharedState;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tracing::{error, info};

pub fn capture(
    tx: mpsc::Sender<Vec<f32>>,
    stop_rx: Receiver<()>,
) -> Result<(), anyhow::Error> {
    info!("Starting audio capture");
    let host = cpal::default_host();
    let input_device = host.default_input_device().expect("no input device available");
    info!("Using input device: {}", input_device.name()?);
    let input_config = input_device.default_input_config().unwrap();
    info!("Input config: {:?}", input_config);

    let input_stream = input_device.build_input_stream(
        &input_config.config(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            if let Err(e) = tx.blocking_send(data.to_vec()) {
                error!("Failed to send audio data: {}", e);
            }
        },
        |err| {
            error!("an error occurred on the input stream: {}", err);
        },
        None,
    )?;

    input_stream.play()?;
    info!("Audio capture started");

    // Block until a stop signal is received.
    stop_rx.recv()?;

    info!("Stopping audio capture");
    Ok(())
}

pub fn playback(
    jitter_buffer: Arc<Mutex<JitterBuffer>>,
    state: Arc<Mutex<SharedState>>,
    stop_rx: Receiver<()>,
) -> Result<(), anyhow::Error> {
    info!("Starting audio playback");
    let host = cpal::default_host();
    let output_device = host.default_output_device().expect("no output device available");
    info!("Using output device: {}", output_device.name()?);
    let output_config = output_device.default_output_config().unwrap();
    info!("Output config: {:?}", output_config);

    let output_stream = output_device.build_output_stream(
        &output_config.config(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            if let Some(packet) = jitter_buffer.lock().unwrap().get_next_frame() {
                let volume = state.lock().unwrap().format.volume;
                let len = std::cmp::min(data.len(), packet.audio_data.len());
                for i in 0..len {
                    data[i] = packet.audio_data[i] * volume;
                }
            }
        },
        |err| {
            error!("an error occurred on the output stream: {}", err);
        },
        None,
    )?;

    output_stream.play()?;
    info!("Audio playback started");

    // Block until a stop signal is received.
    stop_rx.recv()?;

    info!("Stopping audio playback");
    Ok(())
}
