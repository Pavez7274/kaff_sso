# kaff_sso

`kaff_sso` provides a generic fixed-capacity buffer with heap fallback for both small and large collections.

## Features

* **Inline storage** up to 256 elements in `B8`..`B256` variants.
* **Heap fallback** in `Boxed` for buffers exceeding 256 elements.
* **Zero-copy access** via: `as_slice`, `as_ptr` or `From<String>` with length `>=256`
* Implements `PartialEq`, `Eq`, `PartialOrd`, and `Ord` based on buffer length.
* **UTF-8 specialization** (`type UTF8 = Str<u8>`) with:
  * `Deref<Target = str>` and `AsRef<str>`
  * `From<&str>` and `From<String>`
* **Optional N-API integration** (`feature = "napi"`): `FromNapiValue` support for JavaScript strings.

## Quick Start
### Add to `Cargo.toml`
```toml
[dependencies]
kaff_sso = "0.1"
```

### Basic Usage
```rust
use kaff_sso::Str;

// Inline small buffer
let s: Str<u8> = Str::from(&[1, 2, 3][..]);
assert_eq!(unsafe { s.as_slice() }, &[1, 2, 3]);

// UTF-8 string
use kaff_sso::UTF8;
let s = UTF8::from("hello");
assert_eq!(&*s, "hello");
```

### Enabling N-API
```toml
kaff_sso = { version = "0.1", features = ["napi"] }
```

```rust
#[cfg(feature = "napi")]
use napi::FromNapiValue;

// Now UTF8 implements FromNapiValue
```

## Safety

* `as_slice()` is `unsafe`: the caller must guarantee the internal data is valid.
* Converting `UTF8` to `&str` relies on `unsafe { std::mem::transmute }`.

## License

Licensed under MIT OR Apache-2.0
