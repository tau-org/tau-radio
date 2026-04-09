
[![Compiles on macOS (Intel / Silicon), Ubuntu and NixOS](https://github.com/tau-org/tau-radio/actions/workflows/rust.yml/badge.svg?event=pull_request)](https://github.com/tau-org/tau-radio/actions/workflows/rust.yml)

This project is funded through [NGI Zero Core](https://nlnet.nl/core), a fund established by [NLnet](https://nlnet.nl) with financial support from the European Commission's [Next Generation Internet](https://ngi.eu) program. Learn more at the [NLnet project page](https://nlnet.nl/project/Tau).

[<img src="https://nlnet.nl/logo/banner.png" alt="NLnet foundation logo" width="20%" />](https://nlnet.nl)
[<img src="https://nlnet.nl/image/logos/NGI0_tag.svg" alt="NGI Zero Logo" width="20%" />](https://nlnet.nl/core)

## Usage:
This tool is built for livestreaming audio to a server running an instance of
the [`tau-tower`](https://github.com/tau-org/tau-tower) web-radio server. 
This software allows you to access the audio of an audio device on your system, and transmit its signal directly to this radio stream. 

For convenience, the option to record the current stream is also available,
encoded in the same format as the stream itself. 
The format of the streamed and recorded audio is **ogg opus**.

---

To install:
```bash
$ cargo install --git https://github.com/tau-org/tau-radio
```

The first time using the tool, it will search your system for a config file. 
It looks for it in the path:
```bash
$HOME/.config/tau/config.toml # on macOS
```
or 
```bash
$XDG_CONFIG_HOME/tau/config.toml # on Linux
```

If there is no config file located there, you will be prompted to create one. 

```config.toml
username = "username"
password = "emanresu"

# URL to the remote server ( use IP address if tls = false )
url = "example.com"

# The remote server port where we send the stream
upstream_port = 8001

# Default audio interface on macOS
audio_interface = "BlackHole 2ch"

# On linux
# audio_interface = "pipewire"

# broadcast behind tls/ssl encryption ( recommended )
tls = true
```

If you want to temporarily overwrite the config, you are able to pass arguments.

```bash
# Ex: Uses temporary credentials to overwrite some of the config file, and disables the local recording. 
$ tau-radio \
  --username <username> \
  --password <password> \
  --no-recording
```

### Dependencies

**On macOS** (using Homebrew):
```bash
$ brew install opus libopusenc libogg libshout
```

**On Linux** (using apt):
```bash
$ sudo apt update
$ sudo apt install \
  build-essential autoconf automake libtool pkg-config \
  libjack-dev libasound2-dev libpipewire-0.3-dev \
  libopus-dev libopusenc-dev libopusfile-dev opus-tools \
  libogg-dev libshout-dev
```

