use crate::jitter_buffer::JitterBuffer;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use soundnet_types::SharedState;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

pub fn capture(
    tx: mpsc::Sender<Vec<f32>>,
) -> Result<(), anyhow::Error> {
    let host = cpal::default_host();
    let input_device = host.default_input_device().expect("no input device available");
    let input_config = input_device.default_input_config().unwrap();

    let input_stream = input_device.build_input_stream(
        &input_config.config(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let _ = tx.blocking_send(data.to_vec());
        },
        |err| {
            eprintln!("an error occurred on the input stream: {}", err);
        },
        None,
    )?;

    input_stream.play()?;

    // The stream will run until it's dropped.
    // We need to keep the thread alive, so we'll block here.
    std::thread::park();

    Ok(())
}

pub fn playback(
    jitter_buffer: Arc<Mutex<JitterBuffer>>,
    state: Arc<Mutex<SharedState>>,
) -> Result<(), anyhow::Error> {
    let host = cpal::default_host();
    let output_device = host.default_output_device().expect("no output device available");
    let output_config = output_device.default_output_config().unwrap();

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
            eprintln!("an error occurred on the output stream: {}", err);
        },
        None,
    )?;

    output_stream.play()?;

    // The stream will run until it's dropped.
    // We need to keep the thread alive, so we'll block here.
    std::thread::park();

    Ok(())
}
