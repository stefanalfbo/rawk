# rawk-core

![CI](https://github.com/stefanalfbo/rawk/actions/workflows/ci.yml/badge.svg)
[![codecov](https://codecov.io/github/stefanalfbo/rawk/graph/badge.svg?token=SN66UKWM0A)](https://codecov.io/github/stefanalfbo/rawk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Deps.rs Crate Dependencies (latest)](https://img.shields.io/deps-rs/rawk-core/latest)
![Crates.io Version](https://img.shields.io/crates/v/rawk-core)

<div>
  <p align="center">
    <img src="https://raw.githubusercontent.com/stefanalfbo/rawk/main/assets/rawk-logo.png" alt="rawk logo" /> 
  </p>
</div>

Core implementation of an AWK interpreter, providing token definitions, lexical analysis, parsing, and evaluation. **rawk-core** is a Rust implementation of AWK with the goal of POSIX compatibility. Pronounced rök (Swedish for “smoke”).

**rawk-core** is a low-level interpreter crate. Higher-level CLI handling, file I/O, and argument parsing are expected to live in a separate crate or binary, see [rawk](https://github.com/stefanalfbo/rawk/tree/main/crates/rawk).

## Example

```rust
use rawk_core::awk;

fn main() {
    // Execute a simple AWK program that prints each input line
    let output = awk::execute(
        "{ print }",
        vec!["foo".into(), "bar".into()],
    );

    // Each input line is echoed to the output
    assert_eq!(output, vec!["foo".to_string(), "bar".to_string()]);
}
```

