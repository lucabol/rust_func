cargo build --release --target=x86_64-unknown-linux-musl
copy .\target\x86_64-unknown-linux-musl\release\handler .
copy .\host.json.azure .\host.json
func azure functionapp publish rust-mandelbrot
