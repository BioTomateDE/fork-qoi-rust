# [qoi](https://crates.io/crates/qoi)

[![Build](https://github.com/aldanor/qoi-rust/workflows/CI/badge.svg)](https://github.com/aldanor/qoi-rust/actions?query=branch%3Amaster)
[![Latest Version](https://img.shields.io/crates/v/qoi.svg)](https://crates.io/crates/qoi)
[![Documentation](https://img.shields.io/docsrs/qoi)](https://docs.rs/qoi)
[![Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance)

Fork of the rust crate `qoi` ([GitHub](https://github.com/aldanor/qoi-rust), [Docs](https://docs.rs/qoi/latest/qoi/)) to accommodate GameMaker's custom QOI image format.
This is not part of the official QOI format (which is why I needed to fork this crate).
Only RGBA and sRGB are supported since this is what GameMaker uses and the channel count / colorspace aren't even stored in the header.
The header has been changed to fit GameMaker's custom format:
 - Byte 0: `'f'`
 - Byte 1: `'i'`
 - Byte 2: `'o'`
 - Byte 3: `'q'`
 - Byte 4: `width & 0xFF`
 - Byte 5: `(width >> 8) & 0xFF`
 - Byte 6: `height & 0xFF`
 - Byte 7: `(height >> 8) & 0xFF`
 - Byte 8: `length & 0xFF`
 - Byte 9: `(length >> 8) & 0xFF`
 - Byte 10: `(length >> 16) & 0xFF`
 - Byte 11: `(length >> 24) & 0xFF`

Header as a struct (following the [QOI format's Wikipedia page](https://en.wikipedia.org/wiki/QOI_(image_format))):
```c
qoi_header {
    char magic[4]; // magic bytes "fioq"
    uint16_t width; // image width in pixels (LE)
    uint16_t height; // image height in pixels (LE)
    uint32_t length; // image data length in bytes (LE)
};
```

### Examples

```rust
use qoi::{encode_to_vec, decode_to_vec};

let encoded = encode_to_vec(&pixels, width, height)?;
let (header, decoded) = decode_to_vec(&encoded)?;

assert_eq!(header.width, width);
assert_eq!(header.height, height);
assert_eq!(decoded, pixels);
```

### Benchmarks

```
             decode:Mp/s  encode:Mp/s  decode:MB/s  encode:MB/s
qoi.h              282.9        225.3        978.3        778.9
qoi-rust           427.4        290.0       1477.7       1002.9
```

- Reference C implementation:
  [phoboslab/qoi@8d35d93](https://github.com/phoboslab/qoi/commit/8d35d93).
- Benchmark timings were collected on an Apple M1 laptop.
- 2846 images from the suite provided upstream
  ([tarball](https://phoboslab.org/files/qoibench/qoi_benchmark_suite.tar)):
  all pngs except two with broken checksums.
- 1.32 GPixels in total with 4.46 GB of raw pixel data.

Benchmarks have also been run for all of the other Rust implementations
of QOI for comparison purposes and, at the time of writing this document,
this library proved to be the fastest one by a noticeable margin.

### Rust version

The minimum required Rust version for the latest crate version is 1.62.0.

### `no_std`

This crate supports `no_std` mode. By default, std is enabled via the `std`
feature. You can deactivate the `default-features` to target core instead.
In that case anything related to `std::io`, `std::error::Error` and heap
allocations is disabled. There is an additional `alloc` feature that can
be activated to bring back the support for heap allocations.

### License

This project is dual-licensed under MIT and Apache 2.0.
