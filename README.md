# run-length

A pure-Rust library for run-length encoding and its variants — basic RLE, PackBits, modified RLE for binary data, and adaptive RLE with statistics.

## Features

- **Basic RLE** — Simple run-length encoding for byte data
- **PackBits** — Apple's PackBits algorithm for efficient encoding of mixed data
- **Binary RLE** — Modified RLE optimized for binary (0/1) data and images
- **Adaptive RLE** — Dynamically adjusts encoding strategy based on data characteristics
- **Statistics** — Compression ratio analysis, expansion tracking, data profiling

## Usage

```rust
use run_length::{rle, packbits, binary, stats};

let data = b"AAAAAABBBBBB";
let encoded = rle::encode(data);
let decoded = rle::decode(&encoded);
assert_eq!(data.as_slice(), decoded.as_slice());

let ratio = stats::compression_ratio(data.len(), encoded.len());
println!("Compression ratio: {:.2}", ratio);
```

## Modules

- `rle` — Basic run-length encoding/decoding
- `packbits` — PackBits algorithm implementation
- `binary` — Binary data optimized RLE
- `adaptive` — Adaptive RLE encoding
- `stats` — Compression statistics and analysis

## License

MIT
