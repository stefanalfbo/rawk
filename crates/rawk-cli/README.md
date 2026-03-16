# rawk-cli

[![CI](https://github.com/stefanalfbo/rawk/actions/workflows/ci.yml/badge.svg)](https://github.com/stefanalfbo/rawk/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/stefanalfbo/rawk/graph/badge.svg?token=SN66UKWM0A)](https://codecov.io/github/stefanalfbo/rawk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

<div>
  <p align="center">
    <img src="https://raw.githubusercontent.com/stefanalfbo/rawk/main/assets/rawk-logo.png" alt="rawk logo" /> 
  </p>
</div>

Command-line interface for the rawk AWK interpreter. This crate wires `rawk-core` into a `rawk` binary and handles argument parsing, script loading, and input file processing.

## Installation

### From crates.io

Install the `rawk` binary using cargo:

```bash
cargo install rawk-cli
```

Then use the `rawk` command:

```bash
rawk --version
```

### From source

Clone the repository and build from the workspace:

```bash
git clone https://github.com/stefanalfbo/rawk.git
cd rawk
cargo install --path crates/rawk-cli
```

## Usage

Provide a program and an input file:

```bash
rawk "{ print $1 }" input.txt
```

Read the program from a file and pass the input file:

```bash
rawk -f program.awk input.txt
```
