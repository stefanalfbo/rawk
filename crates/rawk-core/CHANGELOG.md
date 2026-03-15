# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.2](https://github.com/stefanalfbo/rawk/compare/v0.3.1...v0.3.2) - 2026-03-15

### Added

- update field splitting logic to use regex and add test for regex field separator
- update regex matching in evaluator and add dynamic regex test; enable additional test cases
- add support for exclamation mark in parser; implement test for negated pattern action
- refactor eval_assignment_infix to handle field assignments and improve readability; add test for chained field assignment
- implement match function in evaluator and update parser to support it; add tests for match functionality
- update trandk test to include in test suite
- enhance comparison handling in evaluator; introduce ComparisonOperand struct and update related logic
- improve output handling in evaluator; ensure consistent record separator usage and update expected test outputs
- refactor output handling in evaluator; improve output record separator logic and update tests
- normalize output lines and enhance string handling in evaluator; update tests
- update printf handling and enable related tests
- enhance split statement to support optional separator and update related tests

### Other

- add hex digit validation tests for is_hex_digit function
- remove unused t_test_ignore macro from test suite
- Fix test status for tintest2 to enable it in the test suite
- Fix test status for tsub0 in t_tests.rs to enable the test
- enable previously ignored test for regex matching
- add fallback test for legacy regex matching in eval_match_function

## [0.3.1](https://github.com/stefanalfbo/rawk/compare/v0.3.0...v0.3.1) - 2026-03-10

### Added

- add support for scientific notation in number tokens and update tests
- enable test for 'tjx' case in t_tests
- enable tests for 'index' and 'intest' cases in t_tests
- enhance parser to handle additional token types and update test for 'in' case
- update truthy evaluation and enhance test coverage for 'if' case
- add test for 'i.x' case in t_tests
- enable test for getline1 function in t_tests

### Other

- simplify control statement parsing and add test for 'incr3' case
- Rename all test scripts so the are using the .awk as a file extension.

## [0.3.0](https://github.com/stefanalfbo/rawk/compare/v0.2.0...v0.3.0) - 2026-03-08

### Added

- add test for expression display including not, increment, decrement, and ternary operations
- add tests for Rule display formatting; verify Begin and End rules output
- handle empty lines in split_fields method; add test for NF with non-space FS counting empty records as zero fields
- enhance gsub functionality with improved replacement semantics; update tests to reflect changes
- add Continue statement support in Parser and Evaluator; implement related tests
- add token_is_immediately_after method to Parser; update assignment and expression parsing to use Token instead of string literals; add test for identifier followed by spaced parentheses
- add index and membership operators to Evaluator; enhance parsing and testing for new functionality
- implement getline function in Evaluator; update input handling and add tests for input consumption
- enhance Evaluator with array aliases and improve print statement handling; update tests for function calls
- add mathematical built-in functions (log, exp, sin, cos, srand) to evaluator; update parser to recognize new tokens and add tests
- add support for field separators in eval_identifier_expression; update tests to activate previously ignored cases
- add support for 'break' statement; update parser, evaluator, and tests
- add support for logical NOT expression; update parser, evaluator, and tests
- enhance parser to support comma-separated expressions in parentheses; update related tests
- add array post-increment, post-decrement, and delete statements; update parser and evaluator
- add support for user-defined functions and enhance return/exit statements
- enable test for 'e' by changing from ignored to active
- add target parameter to gsub statement; update parser, evaluator, and related tests
- add support for 'do while' statement in parser and evaluator; enable related tests
- enable test for 'else' by changing from ignored to active
- enable tests for 'pat', 'pipe', and 'pp' by changing from ignored to active
- add 'sub' statement support in parser and evaluator; enable related tests
- enable test for 'vf' by changing from ignored to active
- enable test for 'format4' by changing from ignored to active
- enable test for 'for3' by changing from ignored to active; update parser to handle newlines
- enable test for 'for2' by changing from ignored to active
- add support for 'Next' statement in parser and evaluator; enable related tests
- enable tests for 'f', 'f.x', 'f0', 'f1', 'f2', 'f3', 'f4', and 'for'; change from ignored to active
- add Split statement support in parser and evaluator; enable related test
- enable test for 'taeiouy' by changing from ignored to active
- add regex support in evaluator and update related tests; enable previously ignored test for vowel matching
- add sqrt function support and enable related tests; refactor numeric parsing in evaluator
- enable test for 'ta' by changing from ignored to active
- refactor numeric expression evaluation and update test for t6
- enable tests for t6a and tbx by removing ignore status
- add Empty statement variant and handle in parser and evaluator; update tests for new behavior
- update test cases to enable t6x, t8x, and t8y in t_tests
- update NF variable handling in evaluator and add test case for NF
- enhance evaluator to support rule evaluation per input line; refactor eval_rule method
- add support for unary plus and minus operators in primary expression parsing; update test case for substr1
- implement eval_printf_argument method and update eval_printf to use it; enhance parser for compound assignment; enable t_test for ttime
- add pre and post increment/decrement expressions and update parser and evaluator

### Fixed

- update FILENAME handling in Evaluator; enable previously ignored test for 'tbe'
- improve number formatting in format_awk_number function; update tests to activate previously ignored cases
- enable test case for tvf1 and replace t_test_ignore with t_test

### Other

- update Program structure to use Action instead of Rule for begin and end blocks; simplify related methods in Evaluator and Parser
- add tests for illegal tokens in Lexer; cover unsupported characters, unterminated regex, and bare dot
- streamline Lexer and Parser token handling; remove allow_regex flag and simplify token retrieval methods
- enable previously ignored tests for various functions in t_tests
- enable previously ignored test for 'tbeginnext'
- enable previously ignored test for 'tbug1'
- enable previously ignored test for average calculation
- Add expected output files for onetrueawk tests with updated data
- Update test cases to enable t_test for tvf3 and tx, replacing t_test_ignore
- Update test cases to enable t4x and t5x, replacing t_test_ignore with t_test
- Enable test case for t4 and update to use t_test macro

## [0.2.0](https://github.com/stefanalfbo/rawk/compare/v0.1.0...v0.2.0) - 2026-03-02

### Other

- Add expected output for t.3 test case
- Add expected output for t.2 and update test data
- Update test cases to enable t2x and t3, replacing t_test_ignore with t_test
- Refactor evaluator to remove unnecessary empty part handling and update expected output files for consistency
- Enable test case for t1x and update to use t_test macro
- Update t_tests to enable test case for t1 and add expected output for t.1
- Add expected output for t.0a and update test case to use t_test macro
- Add test data and test cases for onetrueawk functionality
- Add expected output files for various test cases in onetrueawk-testdata
- Add various test scripts from one true awk repository
- Add SplitAssignment and IfElse statements, enhance evaluator and parser, and introduce tests for table formatting

## [0.1.0](https://github.com/stefanalfbo/rawk/compare/v0.0.8...v0.1.0) - 2026-03-01

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
