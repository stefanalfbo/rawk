use rawk_core::lexer::Lexer;
use rawk_core::parser::Parser;

fn main() {
    let awk_script = std::env::args()
        .nth(1)
        .expect("Please provide an AWK script");
    let path = std::env::args().nth(2).expect("Please provide a file path");

    // println!("AWK Script: {}", awk_script);
    // println!("File Path: {}", path);

    execute(&awk_script, &path)
}

fn execute(awk_script: &str, _path: &str) {
    let lexer = Lexer::new(awk_script);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    println!("Parsed Program: {}", program);
}
