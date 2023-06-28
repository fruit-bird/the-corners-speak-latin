# Hollow CLI
*Hollow shell, hell shallow, branded like cattle*

## Usage
```sh
cargo run --bin=hollow-cli --release --
```
```
Usage: hollow-cli [OPTIONS] [FIRST_TOPIC] [SECOND_TOPIC]

Arguments:
  [FIRST_TOPIC]   Wikipedia topic/link to any article [default: Rumpelstiltskin]
  [SECOND_TOPIC]  Wikipedia topic/link to another article [default: "Moon landing conspiracies"]

Options:
  -l, --lang <SECOND_LANGUAGE>  Language to mix into the articles [default: ja]
  -c, --clipboard               Copy the output to the clipboard
  -h, --help                    Print help
```

## Changelog
- 0.1.1: Added the clipboard feature, to copy output to the clipboard
