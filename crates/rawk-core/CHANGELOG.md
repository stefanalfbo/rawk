# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.2](https://github.com/stefanalfbo/rawk/compare/v0.0.1...v0.0.2) - 2025-12-25

### Other

- Update the README
- Handle line continuation ('\') in the lexer
- Extend the check of white spaces with tab and carriage return
- Handle comments when parsing awk code
- Update code for parsing number so it can handle decimal numbers too.
- Add support for two character tokens in the lexer.
- Handle numbers in the lexer
- Refactor the match expression to parse the identifier
- Define a function to check if a u8 represents a digit (0-9)
- Add unit tests for is_ascii_alphabetic and is_whitespace functions
- Skip white spaces in the lexer while it's scanning the input
- Add keywords to the lexer
