# minilz4

[![Crates.io](https://img.shields.io/crates/v/minilz4)](https://crates.io/crates/minilz4)
[![Docs.rs](https://docs.rs/minilz4/badge.svg)](https://docs.rs/minilz4)

Minimal interface for the [LZ4 compression library](https://github.com/lz4/lz4) frame format.

Links to LZ4 0.9.3.

## Usage

### Examples

#### Simple

```rust
use minilz4::{Encode, EncoderBuilder, Decode};
use std::io::Cursor;

let data = "Blushing is the color of virtue.";

let encoded = Cursor::new(data).encode(&EncoderBuilder::new()).unwrap();
let decoded = Cursor::new(encoded).decode().unwrap();
```

#### Read & Write Traits

```rust
use minilz4::{EncoderBuilder, Decoder};
use std::io::{Cursor, copy};

let data = "Blushing is the color of virtue.";

let mut encoder = EncoderBuilder::new().build(Vec::new()).unwrap();
copy(&mut Cursor::new(data.as_bytes()), &mut encoder).unwrap();
let encoded = encoder.finish().unwrap();

let mut decoder = Decoder::new(Cursor::new(encoded)).unwrap();
let mut decoded = Vec::new();
decoder.read_to_end(&mut decoded).unwrap();
```
