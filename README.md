# c0nst

[![Crates.io](https://img.shields.io/crates/v/c0nst.svg)](https://crates.io/crates/c0nst)
[![Documentation](https://docs.rs/c0nst/badge.svg)](https://docs.rs/c0nst)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)

A procedural macro that enables library authors to write **const traits once**
and automatically provides compatibility for both stable and nightly Rust
consumers.

## The Problem

Embedded development desperately needs const traits for compile-time
computation, and nightly Rust now provides critical const trait support. Many
embedded applications are willing to use nightly since they control their
compiler toolchain.

However, **library authors face a dilemma**:

1. On the one hand, if they implement const traits for nightly consumers, their
   library becomes nightly only and stable consumers are left out.

2. On the other hand, if they don't implement const traits for nightly
   consumers, then nightly consumers are left out.

3. On the other (third?) hand, if they implement both there is significant code
   duplication.

## The Solution

`c0nst` bridges this gap by allowing library authors to **write const traits**
that automatically convert to non-const on stable builds.

## Quick Example

Write your trait once with `c0nst` annotations:

```rust
use c0nst::c0nst;

#[c0nst]
trait Compute {
    fn calculate(&self, x: u32) -> u32;
}

#[c0nst]
impl Compute for u32 {
    fn calculate(&self, x: u32) -> u32 {
        *self + x
    }
}
```

**On stable Rust** (default), this works as regular traits:

```rust
trait Compute {
    fn calculate(&self, x: u32) -> u32;
}

impl Compute for u32 {
    fn calculate(&self, x: u32) -> u32 {
        *self + x
    }
}
```

**On nightly Rust** (with `--features nightly`), this becomes:

```rust
const trait Compute {
    fn calculate(&self, x: u32) -> u32;
}

impl const Compute for u32 {
    fn calculate(&self, x: u32) -> u32 {
        *self + x
    }
}
```

Much more complex scenarios are supported. See the crate documentation.

## Usage Guide

### 1. Library Authors

**Set up your library** with the nightly feature:

```toml
[dependencies]
c0nst = "0.1"

[features]
nightly = ["c0nst/nightly"]  # Enable nightly transformations
```

**Write traits once** using `c0nst` syntax:

```rust
use c0nst::c0nst;

#[c0nst]
trait Default {
    fn default() -> Self;
}

#[c0nst]
impl Default for () {
    fn default() -> Self {}
}

// Use conditional const bounds for generic code
#[c0nst]
fn create_default<T: ?c0nst<Default>>() -> T {
    T::default()
}
```

### 2. Stable Rust Consumers

Just use your library normally:

```toml
[dependencies]
your-const-library = "1.0"
```

```rust
// Works on stable - no const behavior, but same API
let value = MyStruct::default();
```

### 3. Nightly Rust Consumers

Enable the nightly feature to get const trait behavior:

```toml
[dependencies]
your-const-library = { version = "1.0", features = ["nightly"] }
```

```rust
#![feature(const_trait_impl)]

// Now works in const contexts!
const VALUE: MyStruct = MyStruct::default();
```

## License

Licensed under the [MIT license](LICENSE-MIT).
