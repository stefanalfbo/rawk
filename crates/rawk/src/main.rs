use rawk_core::evaluator::Evaluator;
use rawk_core::lexer::Lexer;
use rawk_core::parser::Parser;

fn main() {
    let awk_script = std::env::args()
        .nth(1)
        .expect("Please provide an AWK script");
    let path = std::env::args().nth(2).expect("Please provide a file path");

    execute(&awk_script, &path)
}

fn execute(awk_script: &str, path: &str) {
    let input_lines = std::fs::read_to_string(path)
        .expect("Failed to read input file")
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    let lexer = Lexer::new(awk_script);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    let mut evaluator = Evaluator::new(program, input_lines);

    let output_lines = evaluator.eval();

    for line in output_lines {
        print!("{}", line);
    }
}
