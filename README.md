# c0nst

[![Crates.io](https://img.shields.io/crates/v/c0nst.svg)](https://crates.io/crates/c0nst)
[![Documentation](https://docs.rs/c0nst/badge.svg)](https://docs.rs/c0nst)
[![Build Status](https://github.com/npmccallum/c0nst/workflows/CI/badge.svg)](https://github.com/npmccallum/c0nst/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust 1.63+](https://img.shields.io/badge/rust-1.63+-orange.svg)](https://www.rust-lang.org)

Write const trait code once, run on both nightly and stable Rust.

```rust
#![cfg_attr(feature = "nightly", feature(const_trait_impl))]

c0nst::c0nst! {
    pub c0nst trait Default {
        fn default() -> Self;
    }

    impl c0nst Default for () {
        fn default() -> Self {}
    }

    pub c0nst fn default<T: [c0nst] Default>() -> T {
        T::default()
    }
}
```

## Installation

```toml
[dependencies]
c0nst = "0.2"

[features]
nightly = ["c0nst/nightly"]
```

## How it works

Replace `const` with `c0nst` - the macro transforms your code based on feature
flags:

- **With `nightly` feature**: `c0nst` → `const` (modern const trait syntax)
- **Without `nightly` feature**: `c0nst` and `[c0nst]` are removed (stable
  compatibility)

Perfect for **library authors** - write once, let users choose between nightly
const traits or stable compatibility.

## Features

- **Zero-cost** - Simple keyword replacement, no runtime overhead
- **Forward compatible** - Easy migration when const traits stabilize
  (`s/c0nst/const/g`)
- **Lightweight** - Single proc-macro, minimal dependencies
- **Comprehensive** - Handles all syntax from the
  [RFC](https://github.com/rust-lang/rust/issues/143874)

## For Library Authors

Write const-optional traits that work for everyone! First, expose the choice to
your library users:

```toml
# Cargo.toml
[features]
nightly = ["c0nst/nightly"]
```

Then, define and implement const traits using the nightly syntax (with the
`c0nst` variation):

```rust
// src/lib.rs
#![cfg_attr(feature = "nightly", feature(const_trait_impl))]

c0nst::c0nst! {
    pub c0nst trait Compute {
        fn calculate(&self) -> u32;
    }

    impl c0nst Compute for u32 {
        fn calculate(&self) -> u32 { *self * 2 }
    }
}
```

## For Library Users

### Runtime (Stable)

If you want to run on stable rust, use the library like normal. First, add the
dependency:

```toml
# Cargo.toml
[dependencies]
my-lib = "1.0"
```

Then use the dependency.

```rust
// src/main.rs
let value: u32 = 42u32.calculate(); // ✅ Runtime
```

That's it. You can compile on stable rust and get runtime behavior.

### Compile-time (Nightly)

On the other hand, if you want compile-time behavior and are willing to accept
the requirement to compile only on nightly, then just use the `nightly` feature:

```toml
# Cargo.toml
[dependencies]
my-lib = { version = "1.0", features = ["nightly"] }
```

Then, you get compile-time behavior:

```rust
// src/main.rs
#![feature(const_trait_impl)]
const VALUE: u32 = 42u32.calculate(); // ✅ Compile-time
```
