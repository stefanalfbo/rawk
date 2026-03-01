# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.9](https://github.com/stefanalfbo/rawk/compare/v0.0.8...v0.1.0) - 2026-03-01

### Other

- Add tests for unescape_awk_string function to validate escape sequence handling
- Refactor eval_pattern_condition to simplify comma operator handling
- Enable test case p.52 and update expected output
- Add expected output for test case p.51 and enable the test
- Add expected output for test case p.50 and enable the test
- Add System statement support; implement parsing and evaluation; update tests
- Add pre-decrement and post-decrement statements; implement rand expression and update parser, evaluator, and tests
- Add support for ARGC and ARGV in evaluator; update tests and expected output
- Add PrintPipe statement and related functionality; update evaluator, parser, and tests
- Add PrintRedirect statement and related functionality; update parser, evaluator, and tests
- Enable test for p46 in p_tests; update expected output file
- Add support for filename tracking in evaluator and awk; update tests for expected output
- Add support for output record separator in evaluator; update tests for expected output
- Add support for function definitions and argument consumption in parser; update tests for expected output
- Add support for 'for-in' statements in parser and evaluator; enhance tests for expected output
- Add support for array assignment and array add assignment in parser and evaluator; enhance tests for expected output
- Add support for exit statement in parser and evaluator; enhance tests for expected output
- Add support for 'for' statements in parser and evaluator; enhance tests for expected output
- Add support for while loops and post-increment statements in parser and evaluator; enhance tests for expected output
- Add support for 'if' statements in parser and evaluator; enhance tests for expected output
- Enable test p37 by removing the ignored status
- Refactor evaluator to format numbers consistently; update parser to simplify expression parsing; enhance tests for expected output
- Add support for chained assignments in parser and evaluator; enhance tests and expected output
- Add compound assignment support in parser; enhance tests and expected output
- Add Concatenation expression support; enhance parser, evaluator, and tests
- Add FieldAssignment and Substr support; enhance parser, evaluator, and tests
- Add support for Length expressions in parser; update tests and expected output
- Add Length expression support; enhance parser, evaluator, and tests
- Add gsub statement support; enhance parser, evaluator, and tests
- Add expected output for p.28 and enable corresponding test case
- Add expected output for p.27 and enable corresponding test case
- Add support for AddAssignment and PreIncrement statements; enhance parser and evaluator; update tests for new functionality
- Enhance evaluator with variable storage and update assignment handling; improve parser logic for statement endings; add newline handling for CRLF in lexer; enable test case for p.26 and update expected output
- Add expected output for p.25 and enable test case in p_tests

## [0.0.8](https://github.com/stefanalfbo/rawk/compare/v0.0.7...v0.0.8) - 2026-02-26

### Other

- Add current filename tracking and enhance printf formatting; update tests for consistency
- Implement infix pattern parsing in the parser and enhance pattern evaluation in the evaluator; update tests for consistency
- Enhance regex matching in evaluator and parser; add expected output files for tests
- Refactor evaluator to improve logical expression evaluation and update parser precedence for logical operators; adjust test cases for consistency

## [0.0.7](https://github.com/stefanalfbo/rawk/compare/v0.0.6...v0.0.7) - 2026-02-24

### Other

- Refactor regex matching in evaluator and update test cases for p.13 - p.17
- Implement regex match evaluation and update parser for regex operators; enable test case for p.12
- Add regex condition evaluation and update test case for p.11
- Add new test cases for One True AWK functionality
- Add test case for p.10 with corresponding input
- Add test case for p.9 with corresponding input and expected output
- Add comparison operators support in evaluator and implement test case for p.8
- rename onetrueawk_tests to p_tests
- Implement pattern action support in parser and enhance evaluator with comparison operators; add test case for p.7
- Add tab expansion in printf formatting and implement test case for p.5a
- Add support for NR variable in evaluator and implement test case for p.6
- Add assignment statement support in parser and evaluator to be able to use the FS
- Add printf statement support in parser and evaluator, refactor print handling
- Refactor test cases to use a common assertion function for output validation
- Add test case for p.4 and corresponding expected output
- Add test case for p.3 and corresponding expected output
- Add test case for p.2 and corresponding expected output
- Add initial test files which are based on the test suite from one true awk repository
- Refactor the evaluator and add more state to the struct, like the current line that is processed
- Enable support for NR, the current line number
- Make it possible to use number of fields in a field expression
- Enable support for NF, the number of fields
- Handle more complex print statement like fields seperated with comma, '{ print , }'

## [0.0.6](https://github.com/stefanalfbo/rawk/compare/v0.0.5...v0.0.6) - 2026-02-01

### Other

- Handle field expressions, $1 etc
- Implement operator precedence
- Fix warnings that are reported from clippy

## [0.0.5](https://github.com/stefanalfbo/rawk/compare/v0.0.4...v0.0.5) - 2026-01-21

### Other

- Add more tests to the parser to verify infix, parenthesis and concatenation
- Enable expressions in print statements, like: print 1 + 1 or print (3 * 2) + 1
- Evalute END blocks too
- Make a first attempt to implement support for regex
- Add interactive mode to rawk-cli
- Refactor the awk module to try out another api for that module
- Refactor and rename Item to Rule to use AWK terminlogy more in the code.
- Evaluate BEGIN blocks with print statements that has arguments
- Add end blocks to the ast/parser
- Update the READMEs and add one for the rawk-cli crate.

## [0.0.4](https://github.com/stefanalfbo/rawk/compare/v0.0.3...v0.0.4) - 2026-01-18

### Other

- Start adding support for BEGIN blocks in an AWK program
- Don't let awk panic if there are newlines in the script
- Improve the scanning of hex values
- Fix bug where the read_string is consuming one token to much
- Improve string handling by hadnling unterminated strings
- Add new token type, Identifier, to be able to support user defined variables
- Handle hex numbers in the lexer
- Rename the crate rawk to rawk-cli
- Add location support for tokens
- Update the READMEs for the rawk project

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
