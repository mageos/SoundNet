# SoundNet

A low-latency audio streaming server and client for single-board computers.

## Installation

### Prerequisites

- Rust: https://www.rust-lang.org/tools/install
- ALSA development libraries: `sudo apt-get install libasound2-dev` (on Debian-based systems)

### Building

```
cargo build --release
```

### Cross-compiling for ARM (e.g., Raspberry Pi)

1.  Install the ARM toolchain:

    ```
    rustup target add armv7-unknown-linux-gnueabihf
    sudo apt-get install gcc-arm-linux-gnueabihf
    ```

2.  Configure Cargo for cross-compilation by creating a `.cargo/config.toml` file with the following content:

    ```toml
    [target.armv7-unknown-linux-gnueabihf]
    linker = "arm-linux-gnueabihf-gcc"
    ```

3.  Build the application:

    ```
    cargo build --release --target=armv7-unknown-linux-gnueabihf
    ```

## Configuration

SoundNet can be configured using a `soundnet.toml` file. The following options are available:

```toml
# The friendly name of the device.
friendly_name = "SoundNet Device"

# The port for the REST API.
api_port = 8080
```

## Usage

### Server Mode

To run in server mode, which broadcasts audio to the network:

```
./target/release/soundnet server
```

### Client Mode

To run in client mode, which receives audio from the network:

```
./target/release/soundnet client
```

### Controlling with `soundnet-ctl`

The `soundnet-ctl` utility can be used to discover and control SoundNet devices.

#### Discover

To discover devices on the network:

```
./target/release/soundnet-ctl discover
```

See `docs/USAGE.md` for more detailed examples.

## Packaging

A systemd service file is provided in the `packaging` directory. To install and start the service:

```
sudo cp target/release/soundnet /usr/local/bin/
sudo cp packaging/soundnet.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now soundnet
```