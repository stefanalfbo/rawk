use rawk_core::awk;

fn main() {
    let script = std::env::args()
        .nth(1)
        .expect("Please provide an AWK script");
    let path = std::env::args().nth(2).expect("Please provide a file path");

    execute(&script, &path)
}

fn execute(script: &str, path: &str) {
    let input_lines = std::fs::read_to_string(path)
        .expect("Failed to read input file")
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    let output_lines = awk::execute(script, input_lines);

    for line in output_lines {
        println!("{}", line);
    }
}
