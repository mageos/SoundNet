# Usage Examples

This document provides detailed examples for using `soundnet` and `soundnet-ctl`.

## Basic Streaming

To stream audio from one device to another, you will need two instances of `soundnet` running.

**Device 1 (Server):**

```
./target/release/soundnet server
```

**Device 2 (Client):**

```
./target/release/soundnet client
```

Audio from the microphone of Device 1 will now be playing on the speakers of Device 2.

## Controlling Devices with `soundnet-ctl`

### Discovering Devices

To find all SoundNet devices on your network, run:

```
./target/release/soundnet-ctl discover
```

This will output a list of discovered devices with their name, current mode, and API address.

### Changing the Mode

To change the mode of a device, you can use the `set-mode` command (not yet implemented) or the REST API.

Using `curl`:

```
# Set the device to server mode
curl -X POST -H "Content-Type: application/json" -d '{"mode": "server"}' http://[device-ip]:8080/api/v1/mode

# Set the device to client mode
curl -X POST -H "Content-Type: application/json" -d '{"mode": "client"}' http://[device-ip]:8080/api/v1/mode
```

### Adjusting the Volume

To change the playback volume of a client device:

```
curl -X PUT -H "Content-Type: application/json" -d '{"volume": 0.8}' http://[device-ip]:8080/api/v1/volume
```

### Changing the Stream Format

To change the audio format of a server device:

```
curl -X PUT -H "Content-Type: application/json" -d '{"format": {"codec": "opus", "sample_rate": 48000, "bitrate": 64000, "volume": 1.0}}' http://[device-ip]:8080/api/v1/stream/format
```
