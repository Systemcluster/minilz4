# minilz4

[![Crates.io](https://img.shields.io/crates/v/minilz4)](https://crates.io/crates/minilz4)
[![Docs.rs](https://img.shields.io/docsrs/minilz4)](https://docs.rs/minilz4)
[![Tests & Checks](https://img.shields.io/github/actions/workflow/status/Systemcluster/minilz4/tests.yml?label=tests%20%26%20checks)](https://github.com/Systemcluster/minilz4/actions/workflows/tests.yml)

Minimal interface for the [LZ4 compression library](https://github.com/lz4/lz4) frame format.

Links to [LZ4 1.9.4](https://github.com/lz4/lz4/releases/tag/v1.9.4).

## Usage

`minilz4` provides `Encoder` and `Decoder` structs to encode and decode commpressed data in the LZ4 frame format, as well as `Encode` and `Decode` traits with convenience methods for `Read` and `Write` types.

### Dependency

```toml
[dependencies]
minilz4 = "^0.6"
```

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
