# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.3](https://github.com/stefanalfbo/rawk/compare/v0.0.2...v0.0.3) - 2026-01-04

### Other

- Create a facade to simplify the use of the lexer,parser and the evaluator.
- Create a naive evaluator of the awk script, only print and happy path support :-)
- Make lexer and parser public so they can be used outside the library
- Implement parsing for an AWK program like, { print }
- Improve the code by fixing the warnings given by clippy
- Extend the ast module to handle the print action
- Create a the skeleton to start to parse the next token
- Add a add_item function to the Program struct
- Start a main loop in the parser + refactor the creation of the parse (::new)
- Create a parser module with a skeleton parser
- Try to represent a simple AWK script with an abstract syntax tree,  > 5
- Create an ast module with some initial explatory code for an AST that can be used for AWK
- Fix the improvements that clippy suggests for the lexer.rs
- Enable parsing of built-in functions in the lexer module
- Add support for strings in the lexer

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
