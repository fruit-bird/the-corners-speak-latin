# Hollow
*Hollow shell, hell shallow, branded like cattle*

## Usage
```rust
let hollow = Hollow::new("Rumpelstiltskin", "Moon landing conspiracies", "ja");
let body = hollow.run().await?;

println!("{}", body);
```

## Changelog
- 0.3.1: Concurrent joining of async threads
