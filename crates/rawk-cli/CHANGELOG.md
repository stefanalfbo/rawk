# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2](https://github.com/stefanalfbo/rawk/compare/rawk-cli-v0.1.1...rawk-cli-v0.1.2) - 2026-04-03

### Added

- *(evaluator)* add runtime error handling for negative field index access
- *(cli)* add field separator option for input parsing and update related tests

## [0.1.1](https://github.com/stefanalfbo/rawk/compare/rawk-cli-v0.1.0...rawk-cli-v0.1.1) - 2026-03-22

### Added

- add binary configuration for rawk CLI in Cargo.toml

### Fixed

- update rawk binary reference in CLI tests to use correct executable name

### Other

- *(cli)* add test for system statement in script execution
- *(interactive)* add test for parse error handling in interactive mode
- *(rawk-core)* release v0.4.1
- update Awk initialization to handle errors and improve error reporting
- *(rawk-core)* release v0.4.0
- add Crates.io version badge to README for visibility
- update rawk-cli description for clarity and add installation instructions
- release v0.3.2
- release v0.3.1
- release v0.3.0
- release v0.2.0
- Update rawk-core dependency version to 0.1.0
- release v0.0.9
- Add support for filename tracking in evaluator and awk; update tests for expected output
- release v0.0.8
- release v0.0.7
- Add an integration test to verify that the help shows when no args are given
- Add an interactive integration test when script is given as a file
- Add an interactive integration test when script is given as an argument
- Add an integration test with a script with begin, rules and end blocks
- release v0.0.6
- release v0.0.5
- Enable expressions in print statements, like: print 1 + 1 or print (3 * 2) + 1
- Add interactive mode to rawk-cli
- Refactor the awk module to try out another api for that module
- Update the READMEs and add one for the rawk-cli crate.
- release v0.0.4
- Refactor and add clap to the rawk-cli program
- Rename the crate rawk to rawk-cli
