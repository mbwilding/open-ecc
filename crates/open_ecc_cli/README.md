# open_ecc_cli

Cross-platform CLI for controlling Elgato Key Lights.

Communicates with devices directly over their local HTTP API. No Elgato software or cloud connection is required.

## Install

```
cargo install open_ecc_cli
```

## Setup

Save the IP addresses or hostnames of your devices before running any other command:

```
ecc endpoints 192.168.0.50 192.168.0.51
```

Endpoints are stored in the OS user config directory and reused by all subsequent commands.

## Commands

| Command | Alias | Description |
|---|---|---|
| `brightness <0-100>` | `b` | Set brightness |
| `temperature <2900-7000>` | `k` | Set colour temperature in Kelvin |
| `toggle` | `t` | Toggle on/off state |
| `on` | `1` | Turn on |
| `off` | `0` | Turn off |
| `endpoints <ip...>` | `e` | Save device endpoints |
| `wifi` | `w` | Push Wi-Fi configuration |

All commands that target lights run against all configured endpoints in parallel.

## Examples

```
ecc on
ecc off
ecc toggle
ecc brightness 75
ecc temperature 4500
ecc b 50
ecc k 3200
ecc endpoints 192.168.0.50
ecc wifi --ssid MyNetwork --security wpa --passphrase secret
ecc wifi --ssid OpenNetwork --security none
```

Run `ecc --help` or `ecc <command> --help` for full usage details.
