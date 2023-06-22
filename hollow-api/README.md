# Hollow API
*Hollow shell, hell shallow, branded like cattle*

## Usage
```sh
$ cargo run --bin=hollow-api
# SeEk TruTh... http://127.0.0.1:8088
```
```sh
curl -sSX POST -H "Content-Type: application/json" -d '{
  "first": "Rumpelstiltskin",
  "second": "Moon landing conspiracies",
  "language": "ja"
}' http://localhost:8088/hollow
```

## Changelog
- 0.1.2: Added button to copy output to clipboard
- 0.1.1: Added an API frontend
- 0.1.0: Initial API
