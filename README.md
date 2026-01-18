# rawk

[![CI](https://github.com/stefanalfbo/rawk/actions/workflows/ci.yml/badge.svg)](https://github.com/stefanalfbo/rawk/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/stefanalfbo/rawk/graph/badge.svg?token=SN66UKWM0A)](https://codecov.io/github/stefanalfbo/rawk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io Version](https://img.shields.io/crates/v/rawk-core)](https://crates.io/crates/rawk-core)

A **R**ust implementation of **AWK** with the goal of achieving POSIX compatibility. 

<div>
  <p align="center">
    <img src="assets/rawk-logo.png"> 
  </p>
</div>

A Rust implementation of AWK aimed at POSIX compatibility, with a focus on a small, readable core and a practical CLI. The project is split into two crates to keep parsing/execution logic reusable and the command-line interface thin.

[rawk-core](./crates/rawk-core/README.md): The language core, including the AST, parser, and evaluator, suitable for embedding or for building alternative front-ends.

[rawk-cli](./crates/rawk-cli/README.md): The command-line interface that wires rawk-core into a usable `rawk` binary with flags and file/stdin handling.

## Resources

* [POSIX specification](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/awk.html)
* [GoAWK, an AWK interpreter written in Go](https://benhoyt.com/writings/goawk/) - The inspiration for making rawk.
* [goawk](https://github.com/benhoyt/goawk) - The repository for the GoAWK implementation
* [The One True Awk](https://github.com/onetrueawk/awk) - This is the version of awk described in _The AWK Programming Language_, Second Edition, by Al Aho, Brian Kernighan, and Peter Weinberger (Addison-Wesley, 2024, ISBN-13 978-0138269722, ISBN-10 0138269726).
