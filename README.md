# Data Fetcher

A simple `data_fetcher` tool to help with home automation

## Build

### Locally

`make build`, `make build-docker`

### Primetime

`cargo build --release`

## Run

`docker compose up`, or `make run`

### Config

`~/.config/data_fetcher/default.toml`

```toml
trash_location = ""
weather_api = ""
```
