cargo build --release
copy target\release\handler.exe .
copy .\host.json.local .\host.json
func start
