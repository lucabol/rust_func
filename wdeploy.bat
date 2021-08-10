cargo build --release
copy .\target\release\handler.exe .
copy .\whost.json.azure .\host.json
func azure functionapp publish wrustfunc
