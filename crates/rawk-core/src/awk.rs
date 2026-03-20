use crate::{Evaluator, Lexer, ParseError, Parser, Program};

/// High-level wrapper for compiling and running an AWK script.
///
/// This type parses the script once and can be run with different input.
///
/// # Example
/// ```
/// use rawk_core::awk::Awk;
///
/// let awk = Awk::new("{ print }").unwrap();
/// let output = awk.run(vec!["hello world".into()], None);
/// assert_eq!(output, vec!["hello world".to_string()]);
/// ```
pub struct Awk {
    program: Program<'static>,
}

impl Awk {
    /// Parse an AWK script into an executable program.
    ///
    /// The script is stored with a static lifetime to keep the AST valid.
    /// Returns a parse error if the script is not valid AWK according to this parser.
    pub fn new(script: impl Into<String>) -> Result<Self, ParseError<'static>> {
        let script: String = script.into();
        let script: &'static str = Box::leak(script.into_boxed_str());

        let lexer = Lexer::new(script);
        let parser: &'static mut Parser<'static> = Box::leak(Box::new(Parser::new(lexer)));
        let program = parser.try_parse_program()?;

        Ok(Self { program })
    }

    /// Execute the compiled program against the given input lines.
    ///
    /// When `filename` is `None`, `FILENAME` defaults to `"-"`.
    pub fn run(&self, input: Vec<String>, filename: Option<String>) -> Vec<String> {
        let filename = filename.unwrap_or_else(|| "-".to_string());
        let mut evaluator = Evaluator::new(self.program.clone(), input, filename);

        evaluator.eval()
    }
}
