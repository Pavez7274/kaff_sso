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

This crate exposes several `unsafe` interfaces that require careful usage:

* **`unsafe fn as_slice(&self) -> &[E]`**

  * Returns a raw slice constructed from a pointer and length. The caller must ensure:

    1. The buffer contents remain valid for the returned lifetime.
    2. No other mutable references exist while the slice is alive.
    3. The element type `E` has a valid bit-pattern in the first `len()` positions.

* **`unsafe fn as_mut_ptr(&mut self) -> *mut E`**

  * Provides an unchecked mutable pointer. The caller must ensure:

    1. Pointer arithmetic will not overflow or exceed the buffer bounds.
    2. No immutable references alias the same memory.
    3. Any writes via this pointer respect the valid bit-patterns for `E`.

* **`impl AsRef<str>` and `Deref<Target = str>` for `UTF8`**

  * Both use `mem::transmute` to cast a `&[u8]` slice to `&str` without revalidation. The user must guarantee that the contained bytes are valid UTF-8.

* **`From<UTF8> for String`**

  * Consumes the buffer via `String::from_raw_parts(ptr, len, len)`. Ensure that:

    1. The `UTF8` instance holds a contiguous heap buffer (i.e. the `Boxed` variant).
    2. The memory allocation matches what `String` expects (pointer, length, capacity).
    3. No double-drop occurs—once converted, do not use the original `UTF8` again.

### Best Practices

* Prefer the safe APIs (`as_slice` with prior validation via `std::str::from_utf8`) whenever possible.
* Wrap `unsafe` calls in minimal scopes and document the invariants being upheld.
* Do not mix safe and unsafe accesses that could violate Rust’s aliasing rules.

