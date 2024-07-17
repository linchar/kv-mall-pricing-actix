## Test with a client that initiates the root span
cargo run --example client --features awc

## Running otel collector
~/go/bin/otelcol  --config=otel-collector-config.yaml

## Build image
docker buildx build --platform linux/amd64 -t kv-mall-pricing-rust:0.3.0 .