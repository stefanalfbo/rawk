//! Facade for running AWK scripts with rawk-core.
//!
//! This module wires together the lexer, parser, and evaluator so callers only
//! need to provide a program string and input lines. The lexer tokenizes the
//! script, the parser builds an AST `Program`, and the evaluator walks that
//! program against the provided data to produce output lines.
//!
//! Example:
//! ```
//! use rawk_core::awk;
//!
//! let output = awk::execute("{print}", vec!["foo".into(), "bar".into()]);
//! assert_eq!(output, vec!["foo".to_string(), "bar".to_string()]);
//! ```
use crate::{Evaluator, Lexer, Parser};

/// Execute an AWK script against the given input lines and return the output.
///
/// `script` is the raw AWK program text (e.g., `{print}`) and `input`
/// contains the lines to process. The script is lexed, parsed, and then
/// evaluated; the resulting output lines are returned in order.
pub fn execute(script: &str, input: Vec<String>) -> Vec<String> {
    let lexer = Lexer::new(script);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    let mut evaluator = Evaluator::new(program, input);

    evaluator.eval()
}
